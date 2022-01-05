Our code is YMFC-AL based. So we are not a full developers of code. But the PCB Design is developed by us.


Read more about project : https://github.com/Electrics-Eagles/PiElectricsEagles/wiki or www.eeagles.net


** As we don`t have the stablie verison that do it stabilly we will do not had the dev/stabile folders. For now is redudant **


# PiElectricsEagles


Code Style:

1) Write Tidy ,Clean , Commented code
2) Do not do difficult actions in main.rs file
3) Use KISS,DRY , SOLID principa
4) add public function that returns code module verison
5) do not edit other modules if is not requrired.
6) add dependency if you really need it.


**DO NOT COMMIT TARGET FOLDER WITH DOCS AND BUILD RESULT**
Git Ignore file should help





Well motor reversed direction is requred. From betaflight code i found that string : 
```C++
     float signYaw = (getRcDeflection(FD_YAW) < 0 ? 1 : -1) * (mixerConfig()->yaw_motors_reversed ? 1 : -1);
```
Rewrited this code like that : 
```C++ 
int main(int argc,char *argv[])
{
int FD_YAW = -1; 
float signYaw;

if(getRcDeflection(FD_YAW) < 0 ) {
    signYaw=1;
}
else {
     signYaw=-1; 
}

 printf("%lf\n", signYaw);
return 0;
}

int getRcDeflection(int x) {
return x;
}



```
legacy outdated verison is there [PiElectricsEagles-dev-master.zip](https://github.com/Electrics-Eagles/PiElectricsEagles-dev/files/7818303/PiElectricsEagles-dev-master.zip)

[API Level Explain.pdf](https://github.com/Electrics-Eagles/PiElectricsEagles-dev/files/7818307/API.Level.Explain.pdf)
