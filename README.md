[![SWUbanner](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine)

Our code is YMFC-AL based. So we are not a full developers of code. But the PCB Design is developed by us.

# Sorry now in Uni we had really many things to do that means less much time to do project. Is not mean that been forgotten, is just means little less active as usual. Hope we will recover in summmer period.

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



## Contact Us

We would love to hear from you! You can get in touch with us through the following channels:


- Discord: [https://discord.gg/fACmjJN4dR](https://discord.gg/fACmjJN4dR)




# Log result 

![image](https://user-images.githubusercontent.com/20460747/189099203-3d3414b5-c4b4-4147-88d8-fa95045313e2.png)

Long investigation was opened the window that is wrong with drone. By taking board in arduino and in rpi there are visible difference in graphs. The problem is that Acc values are vital for stabilization on Z axis; So we need to fix acceleromether.


# It was an while since last message there but currently plans are: 
1. Update logging system: (maybe make logs selectable )
2. Update gyro system
3. Create normal docs and video-docs 
4. Create software to upload binary to drone
# The second discovery

PID for drone = REAL TIME

[ElectricsEagles V2.0 Signal Discovery.docx](https://github.com/Electrics-Eagles/PiElectricsEagles-dev/files/10530505/ElectricsEagles.V2.0.Signal.Discovery.docx)

# BUILD IS BROKEN THAT FINE... WE ARE SWITCHING TO REAL TIME
