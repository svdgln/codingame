import math._
import scala.util._
import scala.io.StdIn._
import scala.util.control._
import scala.collection.mutable.ArrayBuffer 

/**
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
 
class robot(storage: Array[Int], target: String){
    val robot_storage = storage
    val robot_target = target
    var robot_carrying = ArrayBuffer[sample]()
    
}

class sample(sampleId: Int, carriedBy: Int,health: Int,costs: Array[Int]) {
    val s_sampleId = sampleId
    val s_carriedBy = carriedBy
    val s_health = health
    val s_costs = costs
    val diagnosed = s_health != -1


}
 

object Player extends App {
  val projectCount = readLine.toInt
  for(i <- 0 until projectCount) {
    val Array(a, b, c, d, e) = (readLine split " ").map (_.toInt)
  }

  // game loop

  while(true) {

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
      
      var storage = Array(storageA, storageB, storageC,storageD,storageE)
      val robot = new robot(storage,target)
      robots.append(robot)
    }

    val Array(availableA, availableB, availableC, availableD, availableE) = (readLine split " ").map (_.toInt)
    val sampleCount = readLine.toInt
    
    
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
      
      var cost = Array(costA, costB, costC,costD,costE)
      val sample = new sample(sampleId,carriedBy,health,cost)
      samples.append(sample)
      
       if (carriedBy > -1){
           robots(carriedBy).robot_carrying.append(sample)
       }
      

      
    }
    
    val me = robots(0)
    

    val loop = new Breaks;
    var molekule = ArrayBuffer[String]()
    molekule += "A"
    molekule += "B"
    molekule += "C"
    molekule += "D"
    molekule += "E"
    
    
    
    if(me.robot_carrying.isEmpty){
        goConnect("SAMPLES",2,me.robot_target)
    }
    else{
        val sample = me.robot_carrying(0)
        if(sample.diagnosed==true){
            var neededMolecule: String = null
        loop.breakable{
        for(i <- 0 until 5){
            if(me.robot_storage(i) < sample.s_costs(i)){
                neededMolecule = molekule(i)
                loop.break;   
            }      
            }
            }
        
        if(neededMolecule!=null) {
      goConnect2("MOLECULES",neededMolecule , me.robot_target)
     
        }
        else{
      goConnect("LABORATORY",sample.s_sampleId, me.robot_target)
     
        }
        }
        else{
           goConnect("DIAGNOSIS", sample.s_sampleId, me.robot_target)
            
        }
    }
      
}


  def goConnect(module:String, id:Int, target:String){
    if(module == target) {
      println("CONNECT " + id)
    }
    else {
      println("GOTO " + module)
    }
  }

  def goConnect2(module:String, typeId:String, target:String){
    if(module == target) {
      println("CONNECT " + typeId.toString)
    }
    else {
      println("GOTO " + module)
    }
  }






}