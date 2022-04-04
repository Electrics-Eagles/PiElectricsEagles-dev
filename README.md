[![SWUbanner](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine)

Our code is YMFC-AL based. So we are not a full developers of code. But the PCB Design is developed by us.

# What it is ?
Description on English: https://github.com/Electrics-Eagles/PiElectricsEagles-dev/wiki/Descripiton-of-the-Project-(English)  
Description on Russian: https://github.com/Electrics-Eagles/PiElectricsEagles-dev/wiki/Descripiton-of-the-Project-(Russian)  
Read more about project : https://github.com/Electrics-Eagles/PiElectricsEagles/wiki or www.eeagles.net  






# PiElectricsEagles

Code Style:

1) Write Tidy ,Clean , Commented code
2) Do not do difficult actions in main.rs file
3) Use KISS,DRY , SOLID principa
4) add public function that returns code module verison
5) do not edit other modules if is not requrired.
6) add dependency if you really need it.


**DO NOT COMMIT TARGET FOLDER WITH DOCS AND BUILD RESULT**





Now you can to check all hardware using 
```
pielectricseagles --testing_hardware 
```
Now you can calibrate esc via : 
```
pielectricseagles --calibrate_esc 1 
```
Where one is esc number.
Also you can test esc via:
```
pielectricseagles --esc_test 1 1200
```
Where 1 is its number and 1200 the throllite.

