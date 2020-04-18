import math._
import scala.util._
import scala.io.StdIn._
import scala.util.control._
import scala.collection.mutable.ArrayBuffer

/**
  * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
  **/

/*
storage is number of molecules held by robot
target is modul where the player is
*/
class robot(storage: Array[Int], target: String){ 
  val robot_storage = storage
  val robot_target = target
}

/*
sampleId is unique id for the sample
carriedBy is 0 if you carry it,1 if other robot carries and -1 if nobody carries
health is health point that make us gain from sample
*/
class sample(sampleId: Int, carriedBy: Int,health: Int,costs: Array[Int]) {
  val s_sampleId = sampleId
  val s_carriedBy = carriedBy
  val s_health = health
  val s_costs = costs
}

object Player extends App {
  val projectCount = readLine.toInt
  for(i <- 0 until projectCount) {
    val Array(a, b, c, d, e) = (readLine split " ").map (_.toInt)
  }

  // game loop

  while(true) {
    // a mutable array holds 2 robot
    var robots = ArrayBuffer[robot]()

    for(i <- 0 until 2) {
      val Array(target, _eta, _score, _storageA, _storageB, _storageC, _storageD, _storageE, _expertiseA, _expertiseB, _expertiseC, _expertiseD, _expertiseE) = readLine split " "
      val eta = _eta.toInt
      val score = _score.toInt
      val storageA = _storageA.toInt
      val storageB = _storageB.toInt
      val storageC = _storageC.toInt
      val storageD = _storageD.toInt
      val storageE = _storageE.toInt
      
     //after robots created through loop, they are added to array
      var storage = Array(storageA, storageB, storageC,storageD,storageE)
      val robot = new robot(storage,target)
      robots.append(robot)
    }
   
    val Array(availableA, availableB, availableC, availableD, availableE) = (readLine split " ").map (_.toInt)
    val sampleCount = readLine.toInt

    //a mutable array holds samples.(mutable array is used so that it can have elements changed, added to, or removed later)
    var samples = ArrayBuffer[sample]()

    for(i <- 0 until sampleCount) {
      val Array(_sampleId, _carriedBy, _rank, expertiseGain, _health, _costA, _costB, _costC, _costD, _costE) = readLine split " "
      val sampleId = _sampleId.toInt
      val carriedBy = _carriedBy.toInt
      val rank = _rank.toInt
      val health = _health.toInt
      val costA = _costA.toInt
      val costB = _costB.toInt
      val costC = _costC.toInt
      val costD = _costD.toInt
      val costE = _costE.toInt
      
      //after samples created through loop, they are added to array
      var cost = Array(costA, costB, costC,costD,costE)
      val sample = new sample(sampleId,carriedBy,health,cost)
      samples.append(sample)


    }
    
    
    val me = robots(0)
    var max_health = 0
    var best_sample: sample = null
    val loop = new Breaks;
    //number of molecules we can carry is limited by ten
    var molekule = ArrayBuffer[String]() // array holds "ABCDE" molecules
    molekule += "A"
    molekule += "B"
    molekule += "C"
    molekule += "D"
    molekule += "E"
 
    // loop finds the best sample that has more health
    for (sample <- samples){
      if(sample.s_health > max_health && sample.s_carriedBy!=1){ //if the other robot is not carrying the sample
        max_health = sample.s_health
        best_sample = sample
      }
    }

    // if we don't carry the samples which we found best, it goes DIAGNOSIS module to take the sample
    if(best_sample.s_carriedBy!=0){
      goConnect("DIAGNOSIS", best_sample.s_sampleId, me.robot_target)
     
    }

    // if we already have the sample, we need to go to the MOLECULES module to gather needed molecules from sample.
    else{
      var neededMolecule: String = null
      loop.breakable{
       /*
        Example: 
        if chosen best sample needs AABDD molecules then the costs array will be [2,1,0,2,0] -> [costA,costB,costC,costD,costE]
        This loop compares the number of molecules in the index of the storage for robot and the cost(number needed molecule from sample)
        loop will contunie until robot has all the molecule needed.    
       */
        for(i <- 0 until 5){
          if(me.robot_storage(i) < best_sample.s_costs(i)){  
            neededMolecule = molekule(i)    // molecule = ["A","B","C","D","E"] 
            loop.break;
          }
        }
      }
      
     //if neededMolecule is not null then we should go to the MOLECULE module to take them
      if(neededMolecule!=null) {
        goConnect2("MOLECULES",neededMolecule , me.robot_target)
      }
     // if all needed molecules are taken, we need to go LABORATORY to produce medicine and collect health point
      else{
        goConnect("LABORATORY", best_sample.s_sampleId, me.robot_target)
      }


    }
  }

  /* 
  target is where we are, so if the module is same with target all we need to is CONNECT what we need with the id.
  otherwise we need to GOTO module.
  */
  def goConnect(module:String, id:Int, target:String){
    if(module == target) {
      println("CONNECT " + id)
    }
    else {
      println("GOTO " + module)
    }
  }
 
 /*
  this function does the same action except it takes String as a id istead of integer. For command; example : CONNECT A
 */

  def goConnect2(module:String, typeId:String, target:String){
    if(module == target) {
      println("CONNECT " + typeId.toString)
    }
    else {
      println("GOTO " + module)
    }
  }

}
