
//ヘッダーファイルを指定
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

//定数の定義
#define T_MAX 3.6
#define SCALE 0.1
#define COUNT_MAX 10
#define Lx 32
#define Ly 32
#define MCS_MAX 13000
#define J 1.0
#define HEAT_MAX 3000
#define MAG_FILE "mag_file.dat"
#define CAL_FILE "cal_file.dat"
#define MS_FILE  "ms_file.dat"
#define SPIN_STEPFILE "step_spin.dat"
#define ENG_STEPFILE "step_eng.dat"

//関数一覧
void spin_initialization();
void spin_judge(double);
void mcs_physics(double);
void spin_arrangement(double);
void physics_quentities(double,double);
void date_graph();
double E();

//グローバル変数
double ea,ma,m;


//メイン関数
int main(void){

    int count, mcs;
    double spin[Lx][Ly];
    double T, E_ave, M_ave, m, beta;
    FILE *fmag, *fcal, *fms, *fsmag, *fseng;
    
    // サンプルデータを一時的に書き込むファイルを用意
    fmag=fopen(MAG_FILE,"w");
    fcal=fopen(CAL_FILE,"w");  
    fms=fopen(MS_FILE,"w");
	
    // 各温度ごとのデータをとる
    for( T=SCALE; T<=T_MAX; T+=SCALE ){
        
      printf("\n温度=%f\n",T);
      beta = 1/T;
      
      // 期待値をとる回数
      for(count=1;count<=COUNT_MAX;count++){

	    spin_initialization((*spin)[]);

        E_ave = 0.0; M_ave = 0.0; m = 0.0;
	    fsmag=fopen(SPIN_STEPFILE,"w");
        fseng=fopen(ENG_STEPFILE,"w"); 
        for( mcs = 1 ; mcs <= MCS_MAX ; mcs++ ){
            spin_judge( beta );
          if( mcs > HEAT_MAX ){
              mcs_physics( beta );
          }
        }
	    fclose(fsmag); fclose(fseng);
	
	    physics_quentities(T,beta);
        spin_arrangement(T);

      }
    }   
    fclose(fmag); fclose(fcal); fclose(fms);

    return 0;

}

//スピンの初期状態を決める関数
void spin_initialization(double spin[Lx][Ly])
{

  int x,y;
  double judge;

  srand((unsigned) time(NULL));

  for(y=0;y<Ly;y++){
    for(x=0;x<Lx;x++){  
      
      judge=(double)rand()/(double)RAND_MAX;
      
      if(judge>0.5){
        spin[x][y] = 1.0;
      }else{
        spin[x][y] = - 1.0;
      }
    
    }
  }

}

//スピンを反転させるか決める関数
void spin_judge(double spin[Lx][Ly], double beta){

    int x,y;
    double e,er,dE,Bf,P;
	
    for(y=0;y<Ly;y++){        
      for(x=0;x<Lx;x++){

          e=energy(); er=-e;
          dE=er-e;

          P=(double)rand()/(double)RAND_MAX;
	
    if(dE<=0){
        Bf=1.0;
    }else{
        Bf=exp(-beta*dE);
    }  
 
    if(P<=Bf){
        spin[x][y]*=(-1.0);
    }

      }
    }
    

}

//エネルギーを計算する関数
double energy()
{

    int Xl,Yd,Xr,Yu;
    double energy,sum;

    Xl=(Lx-1+x)%Lx;Yd=(Ly-1+y)%Ly;
    Xr=(x+1)%Lx;Yu=(y+1)%Ly;
    
    sum=spin[Xl][y]+spin[Xr][y]+spin[x][Yd]+spin[x][Yu];
    
    energy=(-J)*spin[x][y]*sum;

    return energy;

}

//mcsごとの物理量を求める関数
void mcs_physics(double beta)
{

    int N=Lx*Ly;
    double e=0.0,s=0.0;
    FILE *f_mag,*CAL

    for(y=0;y<Ly;y++){
      for(x=0;x<Lx;x++){
          e+=E();
          s+=spin[x][y];
      }
    }
    e/=(2.0);//エネルギーの重複を防ぐため(ここがミソ！)
    ma+=s;ea+=e;
    m+=s/(double)N;

    fprintf(f_mag,"%d %f\n",mcs-HEAT_MAX,s);
    fprintf(CAL,"%d %f\n",mcs-HEAT_MAX,e); 

}

//温度ごとの物理量の値を計算する関数
void physics_quentities(double kT,double beta)
{

    int i,j,N;
    double S,E;
    double Ms=0.0,Ck=0.0;

    N=Lx*Ly;
    ea/=(double)(MCS_MAX-HEAT_MAX);
    ma/=(double)(MCS_MAX-HEAT_MAX);  
    m/=(double)(MCS_MAX-HEAT_MAX); 

    f_mag=fopen("step_spin.dat","r");
    CAL=fopen("step_energy.dat","r"); 

    for(mcs=1;mcs<=(MCS_MAX-HEAT_MAX);mcs++){
        fscanf(f_mag,"%d %lf\n",&i,&S); 
        fscanf(CAL,"%d %lf\n",&j,&E);   
        Ck+=((E-ea)*(E-ea));
        Ms+=((ma-S)*(ma-S));
   } 
    Ck/=(double)(N*(MCS_MAX-HEAT_MAX));
    Ck*=(beta*beta);
    Ms/=(double)((MCS_MAX-HEAT_MAX)*(double)N);
    Ms*=(beta);
    
    fprintf(cal,"%.1f %f\n",kT,Ck); 
    fprintf(f_mag,"%.1f %f\n",kT,m); 
    fprintf(ms,"%.1f %f\n",kT,Ms);   
    
    fclose(f_mag);
    fclose(CAL); 
    	   
}

//スピンの配置を図で表す関数
void spin_arrangement(double kT)
{

    FILE *arrange;
  
    arrange=fopen("graphics.dat","a+");

    fprintf(arrange,"温度kT=%f\n",kT);   
    for(y=0;y<Ly;y++){
      for(x=0;x<Lx;x++){ 
        if(spin[x][y]==1.0){
            fprintf(arrange,"○ ");
        }else{
            fprintf(arrange,"● ");
        } 	
      }        
      fprintf(arrange,"\n");
    }        
    fclose(arrange);

}


