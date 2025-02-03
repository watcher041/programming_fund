
void date_graph()
{ 
    FILE *gp, *f_f_mag, *f_cal;
    
    gp = popen("gnuplot -persist","w");    
    f_f_mag=fopen("f_mag.dat","w");
    f_cal=fopen("cal.dat","w"); 
   
    fprintf(gp, " reset\n");
    fprintf(gp, "set grid\n");
    fprintf(gp, "set xtics 0.1\n");
    fprintf(gp, "set tics font \"Helvetica,8\"\n");
    fprintf(gp, "set xlabel \"温度kT\"\n");
    fprintf(gp, "set ylabel \"比熱C/k\"\n");
    fprintf(gp,"set tics font \"Helvetica,8\"\n");
    fprintf(gp, "set title \"比熱の温度変化\"\n");
    fprintf(gp, "plot \"cal.dat\" with lines linetype 1 title \"比熱\"\n");  
    fprintf(gp, "pause 10 \n");
    fprintf(gp, "set ylabel \"磁化率M\"\n");
    fprintf(gp, "set title \"磁化率の温度変化\"\n");
    fprintf(gp, "plot \"f_mag.dat\" with lines linetype 1 title \"磁化率\"\n");  
    fprintf(gp, "pause 10 \n");
    fprintf(gp, "set ylabel \"帯磁率x\"\n");
    fprintf(gp, "set title \"帯磁率の温度変化\"\n");
    fprintf(gp, "plot \"ms.dat\" with lines linetype 1 title \"帯磁率\"\n");
    fprintf(gp, "pause 10\n");
    pclose(gp);

}