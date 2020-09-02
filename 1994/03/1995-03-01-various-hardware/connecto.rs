Power connectors :    (female connectors originate from power supply)
                      (male connectors are on boards/devices)
                               ÃÄÄÄÄÄÂÄÄÄÄÄÂÄÄÄÄÄÂÄÄÄÄÄ´
3.5 drive : only fits one way  ³ +12 ³ GND ³ GND ³ +5  ³   picture : female
                               ÀÄÄÄÄÄÁÄÄÄÄÄÁÄÄÄÄÄÁÄÄÄÄÄÙ  from power supply

                                  /ÄÄÄÄÂÄÄÄÄÄÂÄÄÄÄÄÂÄÄÄÄ\
5.25 drive : only fits one way   / +12 ³ GND ³ GND ³ +5  \    picture : female
                                 ÀÄÄÄÄÄÁÄÄÄÄÄÁÄÄÄÄÄÁÄÄÄÄÄÙ  from power supply


motherboard : usually consists of 2 separate connectors labeled P8 and P9

    ÚÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄÂÄÄÄ¿
    ³PG ³NC ³+12³-12³GND³GND³GND³GND³ -5³ +5³ +5³ +5³    picture : male
    ÔÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍÏÍÍÍ¾  (on board connector)

PG  = Power Good signal  (orange wire)
NC  = Not Connected (though sometimes this is also +5 Volts)
+12 = Yellow wire
-12 = Blue wire
GND = Black wire
-5  = White wire
+5  = Red wire

The colors described here are not a universal standard, but used most of
the time.

Two variaties exist : with flat pins, and with cylindrical pins,
                      which ofcourse don't mix


***********************************************************************
RS-232    (on board connector : male)
          (female : on modem, printer etc..)


picture : female
                                        DTR         RI
    *14   *15   *16   *17   *18   *19   *20   *21   *22   *23   *24   *25
 *1    *2    *3    *4    *5    *6    *7    *8    *9    *10   *11   *12   *13
       TxD   RxD   RTS   CTS   DSR   GND   DCD

the other pins might have some use on old equipment, but I never saw a PC
using any of them.


    DSR   RTS   CTS   RI
    *6    *7    *8    *9              picture : female
 *1    *2    *3    *4    *5
 DCD   RxD   TxD   DTR   GND

the PC has male RS232 connectors, most equipment has female connectors.
terminal printers, Vt100 terminals and stuff like that also have male
connectors.

PC outputs : DTR, RTS and TxD
PC inputs  : RI, DCD, CTS, DSR and RxD

connecting 2 PC's :  (a so called null-modem)
   DTR ->  DSR,DCD           TxD ->  RxD
   RTS ->  CTS               RxD ->  TxD

***********************************************************************

PS/2  keyboard      (on board connector : female)

       .                 1  data
  *         *            2  -
  6         5            3  GND
                         4  +5
*             *          5  CLK
4             3          6  -

    *     *              picture : female
    2     1




standard keyboard    (on board connector : female DIN)
       .
                          1  CLK
                          2  DATA
*             *           3  RESET
3              1          4  GND
                          5  +5
  *         *
  5    *    4             picture : female
       2

***********************************************************************

joystick        (on board connector : female)

    xy2  but2a  x2   but2b  y2
    *9    *10   *11   *12   *13   *14   *15
 *1    *2    *3    *4    *5    *6    *7    *8         picture : female
 xy1  but1a  x1   but1b        y1

joystick buttons are connected between  but?a and but?b
xy?  is the common for  direction  x?  and y?


***********************************************************************

Printer ports   (on board connector : female)

picture : female

   -af   -err  -init -selin gnd   gnd   gnd   gnd   gnd   gnd   gnd   gnd
    *14   *15   *16   *17   *18   *19   *20   *21   *22   *23   *24   *25
 *1    *2    *3    *4    *5    *6    *7    *8    *9    *10   *11   *12   *13
-str   d0    d1    d2    d3    d4    d5    d6    d7   -ack   busy  pend  sel


centronics connector (female : on printer)

-str   1* *19  gnd             -str  : strobe          (output)
 d0    2* *20  gnd             d0-d7 : data lines      (output)
 d1    3* *21  gnd             -ack  : acknowledge     (input)
 d2    4* *22  gnd             busy  : printer is busy (input)
 d3    5* *23  gnd             pend  : paper out       (input)
 d4    6* *24  gnd             sel   : select          (input)
 d5    7* *25  gnd             -af   : autofeed        (output)
 d6    8* *26  gnd             -err  : error           (input)
 d7    9* *27  gnd             -init : reset printer   (output)
-ack  10* *28  gnd             -selin: select printer  (output)
 busy 11* *29  gnd
 pend 12* *30 -pr.ret          -prime = -init
 sel  13* *31 -prime           -pr.ret : prime return, not used on PC
-af   14* *32 -fault           -fault = -err
 N.C. 15* *33  0
 0    16* *34  N.C.            -str : signal new data to printer
 0    17* *35  +5              -ack : printer accepted data
 +5   18* *36 -selin           d0-d7: some printer ports also allow data input



***********************************************************************
Video  (all female on board)
                                ³    NEC :
    *6    *7    *8    *9        ³          *5    *6  *7  *8
 *1    *2    *3    *4    *5     ³          *1    *2  *3  *4
ÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÁÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÂÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄÄ
    herc(MDA)    CGA         EGA      NEC       ³         *6
                   ..... digital RGB ......     ³    * 1       *11
1    GND         GND         GND      GND       ³         *7
2    GND         GND         R'       R         ³    * 2       *12
3    -           R           R        G         ³         *8
4    -           G           G        B         ³    * 3       *13
5    -           B           B        GND       ³         *9
6    intensity   intensity   G'       GND       ³    * 4       *14
7    video       -           B'       hsync     ³         *10
8    hsync       hsync       hsync    vsync     ³    * 5       *15
9    vsync       vsync       vsync              ³

VGA (analog RGB)

 1  R out             9    no pin
 2  G out            10  sync return
 3  B out            11  mon ID bit 0
 4  mon ID bit 2     12  mon ID bit 1
 5  GND              13  h sync
 6  R return         14  v sync
 7  G return         15    reserved
 8  B return


floppy connector
             aanzicht met drive van je af, connector naar je toe

                              rode streep op kabel
                                 ÚÄÄÄÄÄÄÄÄÄÄÄÄ¿
                                 ³ 1         2³ density select
                                 ³            ³
                                 ³ 3         4³
break in print connector    ->   ³     **     ³
                                 ³ 5         6³
                                 ³            ³
                                 ³ 7         8³ index hole      ->
                                 ³            ³
                                 ³ 9        10³ motor enable A
allemaal                         ³            ³
verbonden                        ³11        12³ drive select B
met massa                        ³            ³
                                 ³13        14³ motor enable B
                                 ³            ³
                                 ³15        16³ drive select A
                                 ³            ³
                                 ³17        18³ direction
                                 ³            ³
                                 ³19        20³ step pulse
                                 ³            ³
                                 ³21        22³ write data
                                 ³            ³
                                 ³23        24³ write enable
                                 ³            ³
                                 ³25        26³ track 0         ->
                                 ³            ³
                                 ³27        28³ write protect   ->
                                 ³            ³
                                 ³29        30³ read data       ->
                                 ³            ³
                                 ³31        32³ select head
                                 ³            ³
                                 ³33        34³ disk change
                                 ³            ³
                                 ÀÄÄÄÄÄÄÄÄÄÄÄÄÙ



Game Port DB15-Female
 1      +5V DC
 2      Button 4 (A_PB1)
 3      Position 0(A_X)
 4      GND
 5      GND
 6      Position 1 (A_Y)
 7      Button 5(A_PB2)
 8      +5V DC
 9      +5V DC
10      Button 6 (B_PB1)
11      Position 2(B_X)
12      GND
13      Position 3(B_Y)
14      Button 7 (B_PB2)
15      +5V DC


AT Keyboard Connector 5pin-DIN		Xt Keyboard Connector 5pin-DIN
pin     assignment                      pin     assignment
1       CLK/CTS (open-collector)        1       CLK/CTS (open-collector)
2       RxD/TxD/RTS (open-collector)    2       Keyboard Data
3       N/C                             3       Reset
4       GND                             4       GND
5       +5V                             5       +5V


IDE Hard Disk Interface IDC-40 Male
pin     assignment      pin     assignment
1       -Reset          2       GND
3       Data 7          4       Data 8
5       Data 6          6       Data 9
7       Data 5          8       Data 10
9       Data 4          10      Data 11
11      Data 3          12      Data 12
13      Data 2          14      Data 13
15      Data 1          16      Data 14
17      Data 0          18      Data 15
19      GND             20      Key
21      (reserved)      22      GND
23      -IOW            24      GND
25      -IOR            26      GND
27      IO Chrdy        28      Ale
29      (reserved)      30      GND
31      IRQ14           32      -IOCS16
33      Addr 1          34      (reserved)
35      Addr 0          36      Addr 2
37      -CS0 (1F0-1F7)  38      -CS1 (3f6-3f7)
39      -Active         40      GND


ESDI Hard Disk Interface IDC-34 Male, IDC-20 Male
               ESDI IDC-34
pin     assignment      pin     assignment
1       GND             2       Head Sel 3
3       GND             4       Head Sel 2
5       GND             6       Write Gate
7       GND             8       Config/Stat Data
9       GND             10      Transfer Ack
11      GND             12      Attn
13      GND             14      Head Sel 0
15      GND             16      Sect/Add MK Found
17      GND             18      Head Sel 1
19      GND             20      Index
21      GND             22      Ready
23      GND             24      Trans Req
25      GND             26      Drive Sel 1
27      GND             28      Drive Sel 2
29      GND             30      Drive Sel 3
31      GND             32      Read Gate
33      GND             34      Command Data

               ESDI IDC-20
pin     assignment      pin     assignment
1       Drive Selected  2       Sect/Add MK Found
3       Seek Complete   4       Addr Mark Enable
5       (reserved)      6       GND
7       Write Clk+      8       Write Clk-
9       Cartridge Chng  10      Read Ref Clk+
11      Read Ref Clk-   12      GND
13      NRZ Write Data+ 14      NRZ Write Data-
15      GND             16      GND
17      NRZ Read Data+  18      NRZ Read Data-
19      GND             20      GND



RLL/MFM  Hard Disk Interface IDC-34 Male, IDC-20 Male
             RLL/MFM IDC-34
pin     assignment      pin     assignment
1       GND             2       Head Sel 8
3       GND             4       Head Sel 4
5       GND             6       Write Gate
7       GND             8       Seek Complete
9       GND             10      Track 0
11      GND             12      Write Fault
13      GND             14      Head Sel 1
15      GND             16      (reserved)
17      GND             18      Head Sel 2
19      GND             20      Index
21      GND             22      Ready
23      GND             24      Step
25      GND             26      Drive Sel 1
27      GND             28      Drive Sel 2
29      GND             30      Drive Sel 3
31      GND             32      Drive Sel 4
33      GND             34      Direction In

             RLL/MFM IDC-20
pin     assignment      pin     assignment
1       Drive Selected  2       GND
3       (reserved)      4       GND
5       (reserved)      6       GND
7       (reserved)      8       GND
9       (reserved)      10      (reserved)
11      GND             12      GND
13      Write Data+     14      Write Data-
15      GND             16      GND
17      Read Data+      18      NRZ Read Data-
19      GND             20      GND


     ISA Bus Connector              EISA Bus Connector
     -----------------              ------------------
Back Side       Component Side  Back Side       Component Side
pin assignment |pin assignment |pin assignment |pin assignment
B1  GND        |A1  CHCHK#     |F1  GND        |E1  CMD#
B2  Reset DRV  |A2  SD7        |F2  +5V        |E2  START#
B3  +5V        |A3  SD6        |F3  +5V        |E3  EXRDY
B4  IRQ9       |A4  SD5        |F4  ---        |E4  EX32#
B5  -5V        |A5  SD4        |F5  ---        |E5  GND
B6  DRQ2       |A6  SD3        |F6  ACCESS KEY |E6  ACCESS KEY
B7  -12V       |A7  SD2        |F7  ---        |E7  EX16#
B8  NOWS#      |A8  SD1        |F8  ---        |E8  SLBURST#
B9  +12V       |A9  SD0        |F9  +12V       |E9  MSBURST#
B10 GND        |A10 CHRDY      |F10 M/IO#      |E10 W/R#
B11 SMWTC#     |A11 AEN        |F11 LOCK#      |E11 GND
B12 SMRDC#     |A12 SA19       |F12 (reserved) |E12 (reserved)
B13 IOWC#      |A13 SA18       |F13 GND        |E13 (reserved)
B14 IORC#      |A14 SA17       |F14 (reserved) |E14 (reserved)
B15 DACK3#     |A15 SA16       |F15 BE3#       |E15 GND
B16 DRQ3       |A16 SA15       |F16 ACCESS KEY |E16 ACCESS KEY
B17 DACK1#     |A17 SA14       |F17 BE2#       |E17 BE1#
B18 DRQ1       |A18 SA13       |F18 BE0#       |E18 LA31#
B19 REFRESH#   |A19 SA12       |F19 GND        |E19 GND
B20 BCLK       |A20 SA11       |F20 +5V        |E20 LA30#
B21 IRQ7       |A21 SA10       |F21 LA29#      |E21 LA28#
B22 IRQ6       |A22 SA9        |F22 GND        |E22 LA27#
B23 IRQ5       |A23 SA8        |F23 LA26#      |E23 LA25#
B24 IRQ4       |A24 SA7        |F24 LA24#      |E24 GND
B25 IRQ3       |A25 SA6        |F25 ACCESS KEY |E25 ACCESS KEY
B26 DACK2#     |A26 SA5        |F26 LA16       |E26 LA15
B27 T/C        |A27 SA4        |F27 LA14       |E27 LA13
B28 BALE       |A28 SA3        |F28 +5V        |E28 LA12
B29 +5V        |A29 SA2        |F29 +5V        |E29 LA11
B30 OSC        |A30 SA1        |F30 GND        |E30 GND
B31 GND        |A31 SA0        |F31 LA10       |E31 LA9

                               |H1  LA8        |G1  LA7
D1  M16#       |C1  SBHE#      |H2  LA6        |G2  GND
D2  IO16#      |C2  LA23       |H3  LA5        |G3  LA4
D3  IRQ10      |C3  LA22       |H4  +5V        |G4  LA3
D4  IRQ11      |C4  LA21       |H5  LA2        |G5  GND
D5  IRQ12      |C5  LA20       |H6  ACCESS KEY |G6  ACCESS KEY
D6  IRQ15      |C6  LA19       |H7  D16        |G7  D17
D7  IRQ14      |C7  LA18       |H8  D18        |G8  D19
D8  DACK0#     |C8  LA17       |H9  GND        |G9  D20
D9  DRQ0       |C9  MRDC#      |H10 D21        |G10 D22
D10 DACK5#     |C10 MWTC#      |H11 D23        |G11 GND
D11 DRQ5       |C11 SD8        |H12 D24        |G12 D25
D12 DACK6#     |C12 SD9        |H13 GND        |G13 D26
D13 DRQ6       |C13 SD10       |H14 D27        |G14 D28
D14 DACK7#     |C14 SD11       |H15 ACCESS KEY |G15 ACCESS KEY
D15 DRQ7       |C15 SD12       |H16 D29        |G16 GND
D16 +5V        |C16 SD13       |H17 +5V        |G17 D30
D17 MASTER16#  |C17 SD14       |H18 +5V        |G18 D31
D18 GND        |C18 SD15       |H19 MAKx       |G19 MREQx


VESA Standard Feature Connector
pin     assignment      pin     assignment
1       PB              2       PG
3       PR              4       PI
5       SB              6       SG
7       SR              8       SI
9       Dot Clock       10      Blank
11      HSync           12      VSync
13      GND             14      GND
15      GND             16      GND
17      Ext Video Sel   18      Ext Sync Sel
19      Ext DotClock Sel20      N/C
21      GND             22      GND
23      GND             24      GND
25      N/C             26      N/C

SCSI Connector Pinouts (Single Ended) IDC-50 Male
pin     assignment|pin  assignment|pin  assignment|pin  assignment
01      GND       |02     -DB0    |27     GND     |28     GND
03      GND       |04     -DB1    |29     GND     |30     GND
05      GND       |06     -DB2    |31     GND     |32     -ATN
07      GND       |08     -DB3    |33     GND     |34     GND
09      GND       |10     -DB4    |35     GND     |36     -BSY
11      GND       |12     -DB5    |37     GND     |38     -ACK
13      GND       |14     -DB6    |39     GND     |40     -RST
15      GND       |16     -DB7    |41     GND     |42     -MSG
17      GND       |18     -DBP    |43     GND     |44     -SEL
19      GND       |20     GND     |45     GND     |46     -C/D
21      GND       |22     GND     |47     GND     |48     -REQ
23      GND       |24     GND     |49     GND     |50     -I/O
25      (open)    |26     TERMPWR


SCSI Connector Pinouts (Differential) IDC-50 Male
pin     assignment|pin  assignment|pin  assignment|pin  assignment
01      (open)    |02     GND     |27     GND     |28     GND
03      +DB0      |04     -DB0    |29     +ATN    |30     -ATN
05      +DB1      |06     -DB1    |31     GND     |32     GND
07      +DB2      |08     -DB2    |33     +BSY    |34     -BSY
09      +DB3      |10     -DB3    |35     +ACK    |36     -ACK
11      +DB4      |12     -DB4    |37     +RST    |38     -RST
13      +DB5      |14     -DB5    |39     +MSG    |40     -MSG
15      +DB6      |16     -DB6    |41     +SEL    |42     -SEL
17      +DB7      |18     -DB7    |43     +C/D    |44     -C/D
19      +DBP      |20     -DBP    |45     +REQ    |46     -REQ
21      DIFFSENS  |22     GND     |47     +I/O    |48     -I/O
23      GND       |24     GND     |49     GND     |50     GND
25      TERMPWR   |26     TERMPWR


Macintosh SCSI Connector Pinouts (Single Ended) DB-25S Female
pin    assignment|pin  assignment|pin  assignment|pin  assignment
01     -REQ      |08     -DB0    |14      GND    |20      -DBP        
02     -MSG      |09     GND     |15      -C/D   |21      -DB1        
03     -I/O      |10     -DB3    |16      GND    |22      -DB2        
04     -RST      |11     -DB5    |17      -ATN   |23      -DB4        
05     -ACK      |12     -DB6    |18      GND    |24      GND         
06     -BSY      |13     -DB7    |19      -SEL   |25      NC (TERMPWR)
07     GND  


PCI Cards Universal/3.3V/5V and 32/64 bit
pin assignment |pin assignment |pin assignment |pin assignment
B1  -12V       |A1   TRST#     |B48  AD[10]    |A48  Ground
B2  TCK        |A2  +12V       |B49 Ground     |A49 AD[09]
B3  Ground     |A3  TMS        |B50 (KEYWAY2)  |A50 (KEYWAY2)
B4  TDO        |A4  TDI        |B51 (KEYWAY2)  |A51 (KEYWAY2)
B5  +5V        |A5  +5V        |B52 AD[08]     |A52 C/BE[0]#
B6  +5V        |A6  INTA#      |B53 AD[07]     |A53 +3.3V
B7  INTB#      |A7  INTC#      |B54 +3.3V      |A54 AD[06]
B8  INTD#      |A8  +5V        |B55 AD[05]     |A55 AD[04]
B9  PRSNT1#    |A9  reserved   |B56 AD[03]     |A56 Ground
B10 reserved   |A10 +Vi/o      |B57 Ground     |A57 AD[02]
B11 PRSNT2#    |A11 reserved   |B58 AD[01]     |A58 AD[00]
B12 (KEYWAY1)  |A12 (KEYWAY1)  |B59 Vi/o       |A59 +Vi/o
B13 (KEYWAY1)  |A13 (KEYWAY1)  |B60 ACK64#     |A60 REQ64#
B14 reserved   |A14 reserved   |B61 +5V        |A61 +5V
B15 Ground     |A15 RST#       |B62 +5V        |A62 +5V
B16 CLK        |A16 Vi/o       |B63 reserved   |A63 Ground
B17 Ground     |A17 VNT#       |B64 Ground     |A64 C/BE[7]#
B18 REQ#       |A18 Ground     |B65 C/BE[6]#   |A65 C/BE[5]#
B19 +Vi/o      |A19 reserved   |B66 C/BE[4]#   |A66 +Vi/o
B20 AD[31]     |A20 AD[30]     |B67 Ground     |A67 PAR64
B21 AD[29]     |A21 +3.3V      |B68 AD[63]     |A68 AD[62]
B22 Ground     |A22 AD[28]     |B69 AD[61]     |A69 Ground
B23 AD[27]     |A23 AD[26]     |B70 +Vi/o      |A70 AD[60]
B24 AD[25]     |A24 Ground     |B71 AD[59]     |A71 AD[58]
B25 +3.3V      |A25 AD[24]     |B72 AD[57]     |A72 Ground
B26 C/BE[3]#   |A26 IDSEL      |B73 Ground     |A73 AD[56]
B27 AD[23]     |A27 +3.3V      |B74 AD[55]     |A74 AD[54]
B28 Ground     |A28 AD[22]     |B75 AD[53]     |A75 +Vi/o
B29 AD[21]     |A29 AD[20]     |B76 Ground     |A76 AD[52]
B30 AD[19]     |A30 Ground     |B77 AD[51]     |A77 AD[50]
B31 +3.3V      |A31 AD[18]     |B78 AD[49]     |A78 Ground
B32 AD[17]     |A32 AD[16]     |B79 +Vi/o      |A79 AD[48]
B33 C/BE[2]#   |A33 +3.3V      |B80 AD[47]     |A80 AD[46]
B34 Ground     |A34 FRAME#     |B81 AD{45]     |A81 Ground
B35 IRDY#      |A35 Ground     |B82 Ground     |A82 AD[44]
B36 +3.3V      |A36 TRDY#      |B83 AD[43]     |A83 AD[42]
B37 DEVSEL#    |A37 Ground     |B84 AD[41]     |A84 +Vi/o
B38 Ground     |A38 STOP#      |B85 Ground     |A85 AD[40]
B39 LOCK#      |A39 +3.3V      |B86 AD[39]     |A86 AD[38]
B40 PERR#      |A40 SDONE      |B87 AD[37]     |A87 Ground
B41 +3.3V      |A41 SBO#       |B88 +Vi/o      |A88 AD[36]
B42 SERR#      |A42 Ground     |B89 AD[35]     |A89 AD[34]
B43 +3.3V      |A43 PAR        |B90 AD[33]     |A90 Ground
B44 C/BE[1]#   |A44 AD[15]     |B91 Ground     |A91 AD[32]
B45 AD[14]     |A45 +3.3V      |B92 reserved   |A92 reserved
B46 Ground     |A46 AD[13]     |B93 reserved   |A93 Ground
B47 AD[12]     |A47 AD11]      |B94 Ground     |A94 reserved

Notes:
Pins 63-94 exist on 64 bit PCI implementation only
KEYWAY1 exists on Universal and 3.3V boards, they are Ground on 5V boards
KEYWAY2 exists on Universal and 5V boards, they are Ground on 3.3V boards
+Vi/o is 3.3V on 3.3V boards, 5V on 5V boards, and define signal rails
  on the Universal board.


