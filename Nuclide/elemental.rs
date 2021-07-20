//kj mol
pub const ELECTRON_AFFINITY : [f64;118] = [
 
       72.769   , -48.0    , 59.6326  , -48.0    , 26.989   , 
       121.7763 , -6.8     , 140.9760 , 328.1649 , -116.0   , 
       52.867   , -40.0    , 41.762   , 134.068  , 72.037   , 
       200.410  , 348.575  , -96.0    , 48.383   , 2.37     ,
       18.0     , 7.289    , 50.911   , 65.21    , -50.0    ,
       14.785   , 63.898   , 111.65   , 119.235  , -58.0    ,
       29.061   , 118.935  , 77.65    , 194.958  , 324.536  ,
       -96.0    , 46.884   , 5.023    , 29.60    , 41.806   ,
       88.516   , 72.10    , 53.0     , 100.96   , 110.27   ,
       54.24    , 125.862  , -68.0    , 37.043   , 107.2984 ,
       101.059  , 190.161  , 295.1531 , -77.0    , 45.505   , 
       13.954   , 53.795   , 55.0     , 10.539   , 9.406    , 
       12.45    , 15.63    , 11.20    , 13.22    , 12.670   , 
       33.96    , 32.61    , 30.10    , 99.0     , -1.93    ,
       23.04    , 17.18    , 31.0     , 78.76    , 5.8273   , 
       103.99   , 150.94   , 205.041  , 222.747  , -48.0    , 
       30.8804  , 34.4183  , 90.294   , 136.0    , 233.087  , 
       -68.0    , 46.89    , 9.6485   , 33.77    , 112.72   , 
       53.03    , 50.94    , 45.85    , -48.33   , 9.93     ,
       27.17    , -165.24  , -97.31   , -28.60   , 33.96    ,
       93.91    , -223.22  , -30.04   , f64::NAN , f64::NAN ,
       f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
       151.0    , f64::NAN , 66.6     , f64::NAN , 35.3     , 
       74.9     , 165.9    , 5.40318  ,
       
 ];
 
 
 //function mullikan electro  
 
    
 
 // (Single, double, triple)    


  pub const COVALENT_RADII : [f64;354] = [

      3.20E-11 , 0.0      , 0.0      , 4.60E-11 , 0.0      , 0.0      , 
      1.33E-10 , 1.24E-10 , 0.0      , 1.02E-10 , 9.00E-11 , 8.50E-11 ,
      8.50E-11 , 7.80E-11 , 7.30E-11 , 7.50E-11 , 6.70E-11 , 6.00E-11 ,
      7.10E-11 , 6.0E-11  , 5.4E-11  , 6.30E-11 , 5.70E-11 , 5.30E-11 ,
      6.40E-11 , 5.90E-11 , 5.30E-11 , 6.70E-11 , 9.60E-11 , 0.0      ,
      1.55E-10 , 1.60E-10 , 0.0      , 1.39E-10 , 1.32E-10 , 1.27E-10 ,
      1.26E-10 , 1.13E-10 , 1.11E-10 , 1.16E-10 , 1.07E-10 , 1.02E-10 ,
      1.11E-10 , 1.02E-10 , 9.40E-11 , 1.03E-10 , 9.40E-11 , 9.50E-11 ,
      9.90E-11 , 9.50E-11 , 9.30E-11 , 9.60E-11 , 1.07E-10 , 9.60E-11 ,
      1.96E-10 , 1.93E-10 , 0.0      , 1.71E-10 , 1.47E-10 , 1.33E-10 ,
      1.48E-10 , 1.16E-10 , 1.14E-10 , 1.36E-10 , 1.17E-10 , 1.08E-10 ,
      1.34E-10 , 1.12E-10 , 1.06E-10 , 1.22E-10 , 1.11E-10 , 1.03E-10 ,
      1.19E-10 , 1.05E-10 , 1.03E-10 , 1.16E-10 , 1.09E-10 , 1.02E-10 ,
      1.11E-10 , 1.03E-10 , 9.60E-11 , 1.10E-10 , 1.01E-10 , 1.01E-10 ,
      1.12E-10 , 1.15E-10 , 1.20E-11 , 1.18E-10 , 1.20E-10 , 0.0      ,
      1.24E-10 , 1.17E-10 , 1.21E-11 , 1.21E-10 , 1.11E-10 , 1.14E-10 ,
      1.21E-10 , 1.14E-10 , 1.06E-10 , 1.16E-10 , 1.07E-10 , 1.07E-10 ,
      1.14E-10 , 1.09E-10 , 1.10E-10 , 1.17E-10 , 1.21E-10 , 1.08E-10 ,
      2.10E-10 , 2.02E-10 , 0.0      , 1.85E-10 , 1.57E-10 , 1.39E-10 ,
      1.63E-10 , 1.30E-10 , 1.24E-10 , 1.54E-10 , 1.27E-10 , 1.21E-10 ,
      1.47E-10 , 1.25E-10 , 1.16E-10 , 1.38E-10 , 1.21E-10 , 1.13E-10 ,
      1.28E-10 , 1.20E-10 , 1.10E-10 , 1.25E-10 , 1.14E-10 , 1.03E-10 ,
      1.25E-10 , 1.10E-10 , 1.06E-10 , 1.20E-10 , 1.17E-10 , 1.12E-10 ,
      1.28E-10 , 1.39E-10 , 1.37E-10 , 1.36E-10 , 1.44E-10 , 0.0      ,
      1.42E-10 , 1.36E-10 , 1.46E-10 , 1.40E-10 , 1.30E-10 , 1.32E-10 ,
      1.40E-10 , 1.33E-10 , 1.27E-10 , 1.36E-10 , 1.28E-10 , 1.21E-10 ,
      1.33E-10 , 1.29E-10 , 1.25E-10 , 1.31E-10 , 1.35E-10 , 1.22E-10 ,
      2.32E-10 , 2.09E-10 , 0.0      , 1.96E-10 , 1.61E-10 , 1.49E-10 ,
      
      1.80E-10 , 1.39E-10 , 1.39E-10 , 1.63E-10 , 1.37E-10 , 1.31E-10 ,
      1.76E-10 , 1.38E-10 , 1.28E-10 , 1.74E-10 , 1.37E-10 , 0.0      , 
      1.73E-10 , 1.35E-10 , 0.0      , 1.72E-10 , 1.34E-10 , 0.0      ,
      1.68E-10 , 1.34E-10 , 0.0      , 1.69E-10 , 1.35E-10 , 1.32E-10 ,
      1.68E-10 , 1.35E-10 , 0.0      , 1.67E-10 , 1.33E-10 , 0.0      ,
      1.66E-10 , 1.33E-10 , 0.0      , 1.65E-10 , 1.33E-10 , 0.0      ,
      1.64E-10 , 1.31E-10 , 0.0      , 1.70E-10 , 1.29E-10 , 0.0      ,
      
      1.62E-10 , 1.31E-10 , 1.31E-10 , 1.52E-10 , 1.28E-10 , 1.22E-10 ,
      1.46E-10 , 1.26E-10 , 1.19E-10 , 1.37E-10 , 1.20E-10 , 1.15E-10 ,
      1.31E-10 , 1.19E-10 , 1.10E-10 , 1.29E-10 , 1.16E-10 , 1.09E-10 ,
      1.22E-10 , 1.15E-10 , 1.07E-10 , 1.23E-10 , 1.12E-10 , 1.10E-10 ,
      1.24E-10 , 1.21E-10 , 1.23E-10 , 1.33E-10 , 1.42E-10 , 0.0      ,
      1.44E-10 , 1.42E-10 , 1.50E-10 , 1.44E-10 , 1.35E-10 , 1.37E-10 ,
      1.51E-10 , 1.41E-10 , 1.35E-10 , 1.45E-10 , 1.35E-10 , 1.29E-10 ,
      1.47E-10 , 1.38E-10 , 1.38E-10 , 1.42E-10 , 1.45E-10 , 1.33E-10 ,
      2.23E-10 , 2.18E-10 , 0.0      , 2.01E-10 , 1.73E-10 , 1.59E-10 ,
      
      1.86E-10 , 1.53E-10 , 1.40E-10 , 1.75E-10 , 1.43E-10 , 1.36E-10 ,
      1.69E-10 , 1.38E-10 , 1.29E-10 , 1.70E-10 , 1.34E-10 , 1.18E-10 ,
      1.71E-10 , 1.36E-10 , 1.16E-10 , 1.72E-10 , 1.35E-10 , 0.0      , 
      1.66E-10 , 1.35E-10 , 0.0      , 1.66E-10 , 1.36E-10 , 0.0      ,
      1.68E-10 , 1.39E-10 , 0.0      , 1.68E-10 , 1.40E-10 , 0.0      ,
      1.65E-10 , 1.40E-10 , 0.0      , 1.67E-10 , 0.0      , 0.0      ,
      1.73E-10 , 1.39E-10 , 0.0      , 1.76E-10 , 0.0      , 0.0      ,
      1.61E-10 , 1.41E-10 , 0.0      , 1.57E-10 , 1.40E-10 , 1.31E-10 ,
      1.49E-10 , 1.36E-10 , 1.26E-10 , 1.43E-10 , 1.28E-10 , 1.21E-10 ,
      1.41E-10 , 1.28E-10 , 1.19E-10 , 1.34E-10 , 1.25E-10 , 1.18E-10 , 
      1.29E-10 , 1.25E-10 , 1.13E-10 , 1.28E-10 , 1.16E-10 , 1.12E-10 ,
      1.21E-10 , 1.16E-10 , 1.18E-10 , 1.22E-10 , 1.37E-10 , 1.30E-10 ,
      1.36E-10 , 0.0      , 0.0      , 1.43E-10 , 0.0      , 0.0      ,
      1.62E-10 , 0.0      , 0.0      , 1.75E-10 , 0.0      , 0.0      ,
      1.65E-10 , 0.0      , 0.0      , 1.57E-10 , 0.0      , 0.0      ,
      
     
];

/*
const IONIC_RADII : [f64;118] = [









]*/
//First 3 Ionization energies in kj/mol 
pub const IONIZATION_ENERGIES : [f64;354] = [

      1312.0   , f64::NAN , f64::NAN , 2372.0   , 5250.5   , f64::NAN , 
      520.02   , 7298.1   , 11815.0  , 899.5    , 1757.1   , 14848.7  ,//Be
      800.6    , 2427.1   , 3659.7   , 1086.5   , 2352.6   , 4620.5   ,//C
      1402.3   , 2856.0   , 4578.1   , 1313.9   , 3388.3   , 5300.5   ,//O
      1681.0   , 3374.2   , 6050.4   , 2080.7   , 3952.3   , 6122.0   ,//Ne
      495.8    , 4562.0   , 6910.3   , 737.7    , 1450.7   , 7732.7   ,//Mg
      577.5    , 1816.7   , 2744.8   , 786.5    , 1577.1   , 3231.6   ,//Si
      1011.8   , 1907.0   , 2914.1   , 999.6    , 2252.0   , 3357.0   ,//S
      1251.2   , 2298.0   , 3822.0   , 1520.6   , 2665.8   , 3931.0   ,//Ar
      418.8    , 3052.0   , 4420.0   , 589.8    , 1145.4   , 4912.4   ,//Ca
      633.1    , 1235.0   , 2388.6   , 658.8    , 1309.8   , 2652.5   ,//Ti
      650.9    , 1414.0   , 2830.0   , 652.9    , 1590.6   , 2987.0   ,//Cr
      717.3    , 1509.0   , 3248.0   , 762.5    , 1561.9   , 2957.0   ,//Fe
      760.4    , 1648.0   , 3232.0   , 737.1    , 1753.0   , 3395.0   ,//Ni
      745.5    , 1957.9   , 3555.0   , 906.4    , 1733.3   , 3833.0   ,//Zn
      578.8    , 1979.3   , 2963.0   , 762.0    , 1537.5   , 3302.1   ,//Ge
      947.0    , 1798.0   , 2735.0   , 941.0    , 2045.0   , 2973.7   ,//Se
      1139.9   , 2103.0   , 3470.0   , 1350.8   , 2350.4   , 3565.0   ,//Kr
      403.0    , 2633.0   , 3860.0   , 549.5    , 1064.2   , 4138.0   ,//Sr
      600.0    , 1180.0   , 1980.0   , 640.1    , 1270.0   , 2218.0   ,//Zr
      652.1    , 1380.0   , 2416.0   , 684.3    , 1560.0   , 2618.0   ,//Mo
      702.0    , 1470.0   , 2850.0   , 710.2    , 1620.0   , 2747.0   ,//Ru
      719.7    , 1740.0   , 2997.0   , 804.4    , 1870.0   , 3177.0   ,//Pd
      731.0    , 2070.0   , 3361.0   , 867.8    , 1631.4   , 3616.0   ,//Cd
      558.3    , 1820.7   , 2704.0   , 708.6    , 1411.8   , 2943.0   ,//In
      834.0    , 1594.9   , 2440.0   , 869.3    , 1790.0   , 2698.0   ,//Te
      1008.4   , 1845.9   , 3180.0   , 1170.4   , 2046.4   , 3099.4   ,//Xe
      375.7    , 2234.3   , 3400.0   , 502.9    , 965.2    , 3600.0   ,//Ba
      538.1    , 1067.0   , 1850.3   , 534.4    , 1050.0   , 1949.0   ,
      527.0    , 1020.0   , 2086.0   , 533.1    , 1040.0   , 2130.0   , 
      540.0    , 1050.0   , 2150.0   , 544.5    , 1070.0   , 2260.0   ,//Sa
      547.1    , 1085.0   , 2404.0   , 593.4    , 1170.0   , 1990.0   ,//Gd
      565.8    , 1110.0   , 2114.0   , 573.0    , 1130.0   , 2200.0   ,//Dy
      581.0    , 1140.0   , 2204.0   , 589.3    , 1150.0   , 2194.0   ,//Er
      596.7    , 1160.0   , 2285.0   , 603.4    , 1174.8   , 2417.0   ,//Yb
      523.5    , 1340.0   , 2022.3   , 658.5    , 1440.0   , 2250.0   ,//Hf
      761.0    , 1500.0   , f64::NAN , 770.0    , 1700.0   , f64::NAN ,//W
      760.0    , 1260.0   , 2510.0   , 840.0    , 1600.0   , f64::NAN ,//Os
      880.0    , 1600.0   , f64::NAN , 870.0    , 1791.0   , f64::NAN ,//Pt
      890.1    , 1980.0   , f64::NAN , 1007.1   , 1810.0   , 3300.0   ,//Hg
      589.4    , 1971.0   , 2878.0   , 715.6    , 1450.5   , 3081.5   ,//Pb
      703.0    , 1610.0   , 2466.0   , 812.1    , f64::NAN , f64::NAN ,//Po
      899.003  , f64::NAN , f64::NAN , 1037.0   , f64::NAN , f64::NAN ,//Rn
      380.0    , f64::NAN , f64::NAN , 509.3    , 979.0    , f64::NAN ,//Ra
      499.0    , 1170.0   , f64::NAN , 587.0    , 1110.0   , 1930.0   ,//Th
      568.0    , f64::NAN , f64::NAN , 597.6    , 1420.0   , f64::NAN ,//U
      604.5    , f64::NAN , f64::NAN , 584.7    , f64::NAN , f64::NAN ,//Pu
      578.0    , f64::NAN , f64::NAN , 581.0    , f64::NAN , f64::NAN ,//Cu
      601.0    , f64::NAN , f64::NAN , 608.0    , f64::NAN , f64::NAN ,//Cf
      619.0    , f64::NAN , f64::NAN , 627.0    , f64::NAN , f64::NAN ,//Fm
      635.0    , f64::NAN , f64::NAN , 642.0    , f64::NAN , f64::NAN ,//No
      470.0    , f64::NAN , f64::NAN , 580.0    , f64::NAN , f64::NAN ,//Rf
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,  
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
      f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
                     
];


// Batsanov's approximations
  pub const VAN_DER_WAAL_ISOLATED : [f64;118] =[
  
 1.96E-10 , f64::NAN , 2.72E-10 , 2.32E-10 , 2.05E-10 , 1.85E-10 ,
 1.70E-10 , 1.64E-10 , f64::NAN , f64::NAN , 2.82E-10 , 2.45E-10 ,
 2.47E-10 , 2.25E-10 , 2.09E-10 , 2.00E-10 , f64::NAN , 3.08E-10 ,
 2.77E-10 , 2.64E-10 , 2.52E-10 , f64::NAN , f64::NAN , 2.23E-10 ,
 2.29E-10 , 2.34E-10 , 2.30E-10 , 2.26E-10 , 2.30E-10 , 2.25E-10 ,
 2.38E-10 , 2.23E-10 , f64::NAN , 2.10E-10 , 2.00E-10 , f64::NAN ,       
 3.22E-10 , 2.90E-10 , 2.73E-10 , 2.63E-10 , 2.50E-10 , 2.40E-10 ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , 2.34E-10 , 2.32E-10 ,
 2.44E-10 , 2.34E-10 , 2.33E-10 , 2.30E-10 , 2.15E-10 , f64::NAN ,
 3.38E-10 , 3.05E-10 , 2.86E-10 , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,  
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 2.54E-10 ,  
 2.44E-10 , 2.35E-10 , 2.38E-10 , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , 2.25E-10 , 2.46E-10 , 2.34E-10 , 2.40E-10 , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 2.78E-10 , 
 f64::NAN , 2.80E-10 , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
     
  ];
  
  
 pub  const VAN_DER_WAAL_CRYSTAL : [f64;118] = [
  
 1.10E-10 , f64::NAN , 2.24E-10 , 1.86E-10 , 1.74E-10 , 1.77E-10 ,
 1.64E-10 , 1.58E-10 , 1.46E-10 , f64::NAN , 2.57E-10 , 2.27E-10 ,//Mg
 2.11E-10 , 2.06E-10 , f64::NAN , 1.81E-10 , 1.76E-10 , f64::NAN ,//Ar
 3.00E-10 , 2.61E-10 , 2.28E-10 , 2.14E-10 , 2.03E-10 , 1.97E-10 ,//Cr
 1.96E-10 , 1.96E-10 , 1.95E-10 , 1.94E-10 , 2.00E-10 , 2.02E-10 ,
 2.08E-10,  2.13E-10 , 2.16E-10 , f64::NAN , 1.87E-10 , f64::NAN ,
 3.12E-10 , 2.78E-10 , 2.45E-10 , 2.25E-10 , 2.13E-10 , 2.06E-10 ,
 2.04E-10 , 2.02E-10 , 2.02E-10 , 2.05E-10 , 2.13E-10 , 2.17E-10 ,
 2.24E-10 , 2.29E-10 , 2.33E-10 , f64::NAN , 2.03E-10 , f64::NAN ,
 3.31E-10 , 2.85E-10 , 2.51E-10 , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 2.24E-10 ,
 2.13E-10 , 2.07E-10 , 2.05E-10 , 2.03E-10 , 2.03E-10 , 2.06E-10 ,
 2.13E-10 , 2.17E-10 , 2.25E-10 , 2.36E-10 , 2.42E-10 , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 2.43E-10 ,
 f64::NAN , 2.17E-10 , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN ,
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
 f64::NAN , f64::NAN , f64::NAN , f64::NAN , 
  
  ];
//https://physlab.lums.edu.pk/images/f/f6/Franck_ref2.pdf van der waals source
// Unlike Most Libraries this does  not use Bondi's approximations

 pub const THERMOCHEMICAL_ELECTRO_NEGATIVE : [f64;118] = [
 
  3.04f64 ,  4.42f64 ,  2.17f64 ,  2.42f64 ,  3.04f64 ,  3.15f64 ,
  3.56f64 ,  3.78f64 ,  4.00f64 ,  4.44f64 ,  2.15f64 ,  2.39f64 ,
  2.52f64 ,  2.82f64 ,  3.16f64 ,  3.44f64 ,  3.50f64 ,  3.57f64 ,
  2.07f64 ,  2.20f64 ,  2.35f64 ,  2.23f64 ,  2.08f64 ,  2.12f64 ,
  2.20f64 ,  2.32f64 ,  2.86f64 ,  2.26f64 ,  2.43f64 ,  2.79f64 ,
  3.15f64 ,  3.37f64 ,  3.45f64 ,  3.37f64 ,  2.07f64 ,  2.13f64 ,
  2.52f64 ,  2.05f64 ,  2.59f64 ,  2.47f64 ,  2.82f64 ,  2.68f64 ,
  2.65f64 ,  2.70f64 ,  2.88f64 ,  2.36f64 ,  2.29f64 ,  2.68f64 ,
  3.05f64 ,  3.14f64 ,  3.20f64 ,  3.12f64 ,  1.97f64 ,  2.02f64 ,
  2.49f64 ,  2.61f64 ,  2.24f64 ,  2.11f64 ,  2.24f64 ,  1.90f64 ,
  1.81f64 ,  2.40f64 ,  2.29f64 ,  2.07f64 ,  2.12f64 ,  2.02f64 ,
  2.03f64 ,  1.78f64 ,  2.68f64 ,  2.01f64 ,  2.32f64 ,  2.42f64 ,
  2.59f64 ,  2.59f64 ,  2.72f64 ,  2.79f64 ,  2.98f64 ,  2.81f64 ,
  2.92f64 ,  2.26f64 ,  2.62f64 ,  2.69f64 ,  2.85f64 ,  3.04f64 ,
  3.04f64 ,  2.01f64 ,  2.15f64 ,  2.22f64 ,  2.62f64 ,  2.62f64 ,
  2.33f64 ,  2.45f64 ,  2.35f64 ,  2.22f64 ,  2.28f64 ,  2.31f64 ,
  2.08f64 ,  2.18f64 ,  2.29f64 ,  2.38f64 ,  2.47f64 ,  2.06f64 ,
  2.10f64 ,  2.27f64 ,  2.38f64 ,  2.51f64 ,  2.48f64 ,  2.52f64 ,
  2.66f64 ,  2.73f64 ,  2.83f64 ,  3.03f64 ,  2.49f64 ,  2.57f64 ,
  2.21f64 ,  2.42f64 ,  2.61f64 ,  2.59f64 ,
  ];

 pub const ALLEN_ELECTRO : [f64;118] = [
 
  2.30f64  ,  4.16f64  ,  0.912f64 ,  1.576f64 ,  2.051f64 ,  2.544f64 ,
  3.066f64 ,  3.610f64 ,  4.193f64 ,  4.787f64 ,  0.869f64 ,  1.293f64 ,
  1.613f64 ,  1.916f64 ,  2.253f64 ,  2.589f64 ,  2.869f64 ,  3.24264  ,//Argon
  0.734f64 ,  1.034f64 ,  1.190f64 ,  1.380f64 ,  1.530f64 ,  1.650f64 ,
  1.750f64 ,  1.800f64 ,  1.840f64 ,  1.880f64 ,  1.850f64 ,  1.588f64 ,
  1.756f64 ,  1.994f64 ,  2.211f64 ,  2.424f64 ,  2.685f64 ,  2.966f64 ,//Krypton
  0.706f64 ,  0.963f64 ,  1.120f64 ,  1.320f64 ,  1.410f64 ,  1.470f64 ,
  1.510f64 ,  1.540f64 ,  1.560f64 ,  1.580f64 ,  1.870f64 ,  1.521f64 ,
  1.656f64 ,  1.824f64 ,  1.984f64 ,  2.158f64 ,  2.359f64 ,  2.582f64 ,//Xe
  0.659f64 ,  0.881f64 ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  1.090f64 ,  1.160f64 ,//hf
  1.340f64 ,  1.470f64 ,  1.600f64 ,  1.650f64 ,  1.680f64 ,  1.720f64 ,
  1.920f64 ,  1.765f64 ,  1.789f64 ,  1.854f64 ,  2.010f64 ,  2.190f64 ,
  2.390f64 ,  2.600f64 ,  0.670f64 ,  0.890f64 ,  f64::NAN ,   2.62f64 ,
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN , 
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN , 
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN , 
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN , 
  f64::NAN ,  f64::NAN ,  f64::NAN ,  f64::NAN , 
  ];
  
  //check Francium, and others
pub  const PAULING_ELECTRO : [f64;118] = [
 
  2.20f64 ,  f64::NAN,  0.98f64 ,  1.57f64 ,  2.04f64 ,  2.55f64 ,
  3.04f64 ,  3.44f64 ,  3.98f64 ,  f64::NAN,  0.93f64 ,  1.31f64 ,
  1.61f64 ,  1.90f64 ,  2.19f64 ,  2.58f64 ,  3.16f64 ,  f64::NAN,
  0.82f64 ,  1.00f64 ,  1.36f64 ,  1.54f64 ,  1.63f64 ,  1.66f64 ,
  1.55f64 ,  1.83f64 ,  1.88f64 ,  1.91f64 ,  1.90f64 ,  1.65f64 ,
  1.81f64 ,  2.01f64 ,  2.18f64 ,  2.55f64 ,  2.96f64 ,  3.00f64 ,//kr
  0.82f64 ,  0.95f64 ,  1.22f64 ,  1.33f64 ,  1.60f64 ,  2.16f64 ,
  1.90f64 ,  2.20f64 ,  2.28f64 ,  2.20f64 ,  1.93f64 ,  1.69f64 ,
  1.78f64 ,  1.96f64 ,  2.05f64 ,  2.10f64 ,  2.66f64 ,  2.60f64 ,//Xe
  0.79f64 ,  0.89f64 ,  1.10f64 ,  1.12f64 ,  1.13f64 ,  1.14f64 ,
  1.13f64 ,  1.17f64 ,  1.20f64 ,  1.20f64 ,  1.10f64 ,  1.22f64 ,
  1.23f64 ,  1.24f64 ,  1.25f64 ,  1.10f64 ,  1.27f64 ,  1.30f64 ,
  1.50f64 ,  2.36f64 ,  1.90f64 ,  2.20f64 ,  2.28f64 ,  2.54f64 ,
  2.00f64 ,  1.62f64 ,  2.33f64 ,  2.02f64 ,  2.00f64 ,  2.20f64 ,
  2.20f64 ,  0.79f64 ,  0.90f64 ,  1.10f64 ,  1.30f64 ,  1.50f64 ,
  1.38f64 ,  1.36f64 ,  1.28f64 ,  1.13f64 ,  1.28f64 ,  1.30f64 ,
  1.30f64 ,  1.30f64 ,  1.30f64 ,  1.30f64 ,  1.30f64 ,  1.30f64 ,
  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
  f64::NAN,  f64::NAN,  f64::NAN,  f64::NAN,
  ];