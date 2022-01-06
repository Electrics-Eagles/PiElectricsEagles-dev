//
     float signYaw = (getRcDeflection(FD_YAW) < 0 ? 1 : -1) * (mixerConfig()->yaw_motors_reversed ? 1 : -1);

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



