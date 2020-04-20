# Codingame - Bot Programming
## CODE4LIFE
### Group members: Sevda GÜLEN, Ceren BÜLBÜL

The code used to play matches in the Code4Life contest from [codingame platform](https://www.codingame.com/multiplayer/bot-programming/code4life).
Written in both Scala and Rust Programming Languages.

## Strategies

### Scala Programming Language** :
**Wood***

Main strategy to win the other robot is finding the best sample according its health point. It searchs samples from the created samples array and compare their health point then selects a samples which has max health as a best sample.

- **DIAGNOSIS** : If sample that chosen as a best sample according to the strategy is not carried by our robot then it goes to DIAGNOSIS module and CONNECT sample with sample id.
- **MOLECULES** : Get molecules that needed from the chosen best sample as searcing if we have molecules that costs of sample has. CONNECT molecules.
Then go to LABORATORY
- **LABORATORY** : It takes molecules and sample then produce the medicine.Because we chosed the best sample, we get the highest health point and increase our score.

### Wood to Bronze

In this part we have extra module as SAMPLE which robots will collect samples from it instead of from DIAGNOSIS.So ,sample data could be diagnosed or undiagnosed. In addition, as an extra we have rank:1,2,3.The higher rank gets the higher health point.

- **SAMPLES** :
- **DIAGNOSIS** : 
- **MOLECULES** : 
- **LABORATORY** : 







                  
