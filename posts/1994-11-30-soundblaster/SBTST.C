#include <stdio.h>
#include <conio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <dos.h>

typedef int bool;
#define FALSE 0
#define TRUE  1

// Soundblaster:  0x220
// Adlib:         0x388   == 0x228

int SbAddr=0x220;
int SbIRQ =5;
int SbDMA =1;
int SbType=1;  // ?

#define BUFLEN  64         // for gethex

// FM synthesizer:
//    18 operating cells : capable of producing A*sin(F*t+X)
//    where A=amplitude, F=frequency, X=input
//
// modes of operation
//    - 9 channel FM sound: each channel uses 2 operator cells
//      waveform:  F(t)=A*sin(fc*t + I*sin(fm*t))
//        A = output amplitude
//        I = modulation index (amplitude of modulator)
//        fc= carrier frequency
//        fm= modulator frequency
//    - 6 channel FM sound + 5 percussion sounds
//      bass drum    3+16
//      hi hat       14
//      tom tom      15
//      snare drum   17
//      top cymbal   18
//
// 00     unused
// 01  test lsi / enable waveform control
//     .5 =1 : FMchip controls waveform
// 02  timer 1 data (12500 hz)  -> irq 0  on overflow, stat.76 is set
// 03  timer 2 data (3125 hz)  -> irq 0  on overflow, stat.75 is set
// 04  timer control
//     .7 reset IRQ flag
//     .6 timer 1 disable
//     .5 timer 2 disable
//     .1 timer 2 starts  when set to 1
//     .0 timer 1 starts  when set to 1
// 08  speech synth mode / kebd split note select
//     .7 CSM enable speech synthesizer (all keyon bits must be clear)
//     .6 SEL 0: keyboard split=F.8  1: kbsplit=F.9
// 20+OP_CHA  amp mod vibr egtype keyscaling multiple
//     .7 AM  enable amplitude modulation  -- (freq=3.7Hz)
//     .6 VIB enable vibrato -- (freq=6.4Hz)
//     .5 EG  maintain sustain level until released
//     .4 KSR : RATE=KSRfact*ADSR  KSRfact={0: 4.25,  1: 5}
//     .3210 = MULT = {0.5, 1, 2, .. 10, 10, 12, 12, 15, 15}
//         F(t)=A*sin(MULT*FREQ*t+X)
// 40+OP_CHA  keyscaling level  / operator output level
//    .76 = KSL
//           0, 3, 1.5, 6 dB/OCT  volume/freq scaling
//    .543210 = total level : * 0.75 dB
// 60+OP_CHA  attack rate / decay rate
//    .7654 = attack
//    .3210 = decay
// 80+OP_CHA  sustain level / release rate
//    .7654 = sustain  : sustain*3dB
//    .3210 = release
// a0+CHA     frequency (low 8 bits)
// b0+CHA     keyon / octave / freq (high 2 bits)
//    .5  = KEYON
//    .432= octave of output signal
//    .10 = freq (high 2 bits)
//  Freq = 50000*Freq*2^(octave-20)
//  block=4:  (freq values)
//   cDdEFfGgAaBC = 16b, 181, 198, 1b0, 1ca, 1e5, 202, 220, 241, 263, 287, 2ae
//
// bd         am depth / vibrato depth / rhythm control
//    .7  = AM depth   (0: 1dB, 1: 4.8dB)
//    .6  = VIB depth  (0: 7 cent, 1: 14 cent)
//    .5  = rhythm
//    .4  = bass drum
//    .3  = snare drum
//    .2  = tom tom
//    .1  = top cymbal
//    .0  = hihat
// c0+CHA     feedback strength / connection type
//    .321= feedback factor: 0, 1/16, 1/8, 1/4, 1/2, 1, 2, 4
//          controls modulator feedback
//    .0  = con:
//           0: F(t)=A*sin(fc*t + I*sin(fm*t))
//           1: F(t)=A*sin(fc*t)+I*sin(fm*t))
//
// e0+OP_CHA  wave select
//    .10 = wave sel
//          0: full sinewave
//          1: sine(0 .. pi),  0(pi .. 2pi)
//          2: abs(sin(x))
//          3: abs(sin(x)) for x=0 .. pi/2, pi .. 3pi/2,  zero otherwise
//
//  OP_CHA definition
//  channel:  1  2  3   4  5  6   7  8  9
//  op1       0  1  2   8  9  a  10 11 12  modulator
//  op2       3  4  5   b  c  d  13 14 15  carrier
//
//  CHA  = channel number-1
//
//  fm status register:
//    .7  = IRQ
//    .6  = T1 flag
//    .5  = T2 flag

#define FM_LEFT_STATUS     (SbAddr+0x00)        // Read only
#define FM_LEFT_ADDRESS    (SbAddr+0x00)        // write only
#define FM_LEFT_DATA       (SbAddr+0x01)        // write only

#define FM_RIGHT_STATUS    (SbAddr+0x02)        // Read only
#define FM_RIGHT_ADDRESS   (SbAddr+0x02)        // write only
#define FM_RIGHT_DATA      (SbAddr+0x03)        // write only

#define MIXER_ADDRESS      (SbAddr+0x04)        // 00-3f
//    RESET       0x00      // + write data
//    VOC_VOL     0x04
//    MIC_VOL     0x0a
//    RECORD_SRC  0x0C
//          .5   input filter switch
//          .3   freq (0=low, 1=high)
//          .21  00=MIC, 01=CD, 10=?, 11=LINE
//
//    CHANNELS    0x0e
//          .1    enable stereo output
//          .3    enable hi freq  ANFI filter
//          .5    enable filter
//    MASTER_VOL  0x22
//    FM_VOL      0x26
//    CD_VOL      0x28
//    LINE_VOL    0x2e
#define MIXER_DATA         (SbAddr+0x05)
#define DSP_RESET          (SbAddr+0x06)
//                                +0x07         ??
#define FM_STATUS          (SbAddr+0x08)        // Read only
#define FM_ADDRESS         (SbAddr+0x08)        // write only
#define FM_DATA            (SbAddr+0x09)        // write only
#define DSP_READ_DATA      (SbAddr+0x0a)        // read only
//                                +0x0b         ??
#define DSP_WRITE_DATA     (SbAddr+0x0c)        // write only
#define DSP_WRITE_STATUS   (SbAddr+0x0c)        // read only
//   .7 == DSP busy (cant send data to DSP_WRITE_DATA)
//   .6 == data ready
//                                +0x0d         ??
#define DSP_DATA_AVAIL     (SbAddr+0x0e)        // read only, irq acknowledge
//   .7 == Data ready at DSP_READ_DATA
//   .6 == data ready
//   .2 == ? error
//                                +0x0f         ??
#define CDROM_DATA         (SbAddr+0x10)
#define CDROM_STATUS       (SbAddr+0x11)
#define CDROM_RESET        (SbAddr+0x12)
#define CDROM_ENABLE       (SbAddr+0x13)

// commands for DSP
//  00-0f    // [] -> [byte]
#define DIRECT_8_BIT_DAC    0x10      // [data:byte] -> []
//  11       // [byte byte]  -> []
//  12       // [byte]       -> []
//  13       // [byte byte]  -> []
#define DMA_8_BIT_DAC       0x14      // [size:wordLH] -> []
//  15       // [byte],[data] -> []
#define DMA_2_BIT_DAC       0x16
#define DMA_2_BIT_REF_DAC   0x17
//  18       // [byte]        -> []
//  19       // [word]        -> []
//  1a       // [byte]        -> []
//  1b       // [word]        -> []

//  1c       // [] []
//  1d       // [] []
//  1e       // [] []   (anders)
//  1f       // [] []
#define DIRECT_ADC          0x20      // [] -> [byte]
//  21       // []      -> [wordLH]   16bit adc ??
//  22       // []      -> [byte]
//  23       // []      -> [wordLH]   16bit adc ??
//
#define DMA_ADC             0x24      // [size:wordLH] -> []
//  25       // [wordLH] -> datablock    (get N bytes (16bit data LH))
//  26       // [wordLH] -> datablock    (get N bytes (8bit data))
//  27       // [wordLH] -> datablock    (get N bytes (16bit data LH))
#define MIDI_READ_POLL      0x30      // [] -> [byte]
#define MIDI_READ_IRQ       0x31
#define MIDI_UART_POLL      0x34
#define MIDI_UART_READ      0x35
#define MIDI_WRITE_POLL     0x38      // [byte] -> []
#define TIME_CONSTANT       0x40      // [byte] -> []
#define SET_BLOCKSIZE       0x48      // [size:wordLH] -> []
#define MDAC1               0x61      // [word],[byte] -> []
#define MDAC2               0x62
#define MDAC3               0x63
#define MDAC4               0x64
#define MDAC5               0x65
#define MDAC6               0x66
#define MDAC7               0x67
#define DMA_4_BIT_DAC       0x74
#define DMA_4_BIT_REF_DAC   0x75
#define DMA_26_BIT_DAC      0x76
#define DMA_26_BIT_REF_DAC  0x77
#define DMA_2_BIT_DAC       0x7A
#define DMA_2_BIT_REF_DAC   0x7B
#define SILENCE             0x80
#define HIGH_DMA_8_BIT_DAC  0x91      // [] -> []
#define HIGH_DMA_8_BIT_ADC  0x99      // [] -> []
#define HALT_DMA            0xD0      // [] -> []
#define SPEAKER_ON          0xD1      // [] -> []
#define SPEAKER_OFF         0xD3      // [] -> []
#define CONTINUE_DMA        0xD4      // [] -> []
#define READ_SPEAKER        0xD8
#define DSP_ID              0xE0      // [byte] -> [byte]
#define DSP_VER             0xE1      // [] -> [byte, byte]

// when to use HIGH_DMA
#define MAX_LO_REC          12048
#define MAX_LO_PLAY         22222
//
//  sampling rates:
//    input  8bit  :  4000 - 12000
//    output 8bit  :  4000 - 23000
//    output 4bit  :  4000 - 12000   (ADPCM)
//    output 2.6bit:  4000 - 13000   (ADPCM)
//    output 2bit  :  4000 - 11000   (ADPCM)
//
//  data size = # data bytes -1
//
#define OK                0
#define ERR_TIMEOUT       1
#define CMD_TIMEOUT   10000

typedef struct {
  int x_start;
  int y_start;
  int x_end;
  int y_end;
  int x_cur;
  int y_cur;
  bool showcursor;
} WINDOW;

int getkey(void)
{
  int c;
  c=getch();
  if (kbhit())
    c|=getch()<<8;
  while (kbhit()) getch();   // flush kb buffer
  return c;
}

void hidecursor(void)
{
  union REGS r;
  r.x.ax=0x100;
  r.x.cx=0x2000;
  int86(0x10,&r, &r);
}

void showcursor(void)
{
  union REGS r;
  r.x.ax=0x100;
  r.x.cx=0xb0d;
  int86(0x10,&r, &r);
}

void SetWindow(WINDOW *w)
{
  static WINDOW *current=NULL;
  if (current)
  {
    current->x_cur=wherex();
    current->y_cur=wherey();
  }
  if (w)
  {
    hidecursor();
    window(w->x_start, w->y_start, w->x_end, w->y_end);
    gotoxy(w->x_cur, w->y_cur);
    if (w->showcursor)
      showcursor();
  }
  else
  {
    hidecursor();
    window(1,1, 80, 25);
    gotoxy(1,1);
  }

  current=w;
}

WINDOW *InitWindow(int x0, int y0, int x1, int y1, int showcursor)
{
  WINDOW *w;
  w=(WINDOW *)malloc(sizeof(WINDOW));
  if (w==NULL)
    return NULL;
  w->x_start=x0;
  w->y_start=y0;
  w->x_end=x1;
  w->y_end=y1;
  w->showcursor=showcursor;
  w->x_cur=1;
  w->y_cur=1;
  SetWindow(w);
  clrscr();
  return w;
}

int dsp_cmd(int c)
{
//  int wait=0;
//  while (inportb(DSP_WRITE_STATUS)&0x80)
//  {
//    if (++wait >CMD_TIMEOUT)
//      return ERR_TIMEOUT;
//  }
  outportb(DSP_WRITE_DATA, c);
  return OK;
}

int sendcommand(char *buf, int buflen)
{
  int res;
  while (buflen--)
  {
    res=dsp_cmd(*buf++);
    if (res)
      return res;
  }
  return OK;
}

void print_input(void)
{
  int i=0;
  while (!kbhit() && inportb(DSP_WRITE_STATUS)&0x80)
  {
    if (inportb(DSP_DATA_AVAIL)&0x80)
    {
      if ((i%0x10)==0x00)
        cprintf("input %04x : ", i);
      cprintf(" %02x", inportb(DSP_READ_DATA));
      if ((i%0x10)==0x0f)
        cprintf("\r\n");
      i++;
    }
  }
  if ((i%0x10)==0x0f)
    cprintf("\r\n");
}

void display_mixer(void)
{
  int i;
  for (i=0 ; i<0x40 ; i++)
  {
    outportb(MIXER_ADDRESS, i);
    if ((i%0x10)==0x00)
      cprintf("mixer %04x : ", i);
    cprintf(" %02x", inportb(MIXER_DATA));
    if ((i%0x10)==0x0f)
      cprintf("\r\n");
  }
  if ((i%0x10)==0x0f)
    cprintf("\r\n");
}

// prints SB registers
void print_status(void)
{

  cprintf("L=%02x M=%02x R=%02x  WS=%02x DA=%02x :",
    inportb(FM_LEFT_STATUS), inportb(FM_STATUS), inportb(FM_RIGHT_STATUS),
    inportb(DSP_WRITE_STATUS), inportb(DSP_DATA_AVAIL));
  cprintf("RD=%02x\r\n", inportb(DSP_READ_DATA));
}

int gethex(char *cmd, int n)
{
  char buf[83];
  char *p, *q;
  int i;

  buf[0]=80;

  p=cgets(buf);
  buf[buf[1]+2]=0;

  for (i=0 ; i<n ; i++)
  {
    cmd[i]=strtol(p,&q,16);
    if (q==p) break;
    p=q;
  }
  if (*q=='$')
    while (i<n)  cmd[i++]=0;
  return i;
}

void GetBlasterEnv(void)
{
  char *p;

  p=getenv("BLASTER");
  while (*p)
  {
    switch(*p++)
    {
      case 'a':
      case 'A':
        SbAddr=strtol(p, &p, 16);
        break;
      case 'i':
      case 'I':
        SbIRQ=strtol(p, &p, 10);
        break;
      case 'd':
      case 'D':
        SbDMA=strtol(p, &p, 10);
        break;
      case 't':
      case 'T':
        SbType=strtol(p, &p, 10);
      default:
        break;
    }
  }
}

// THIS FUNCTION DOES NOT REALLY MEASURE USECS !!!!!
void waitusec(int usec)
{
  while (usec--) ;
}

// info in:
//
// sbf3
//   - sbmidi.c    *
//   - getvol.c    *
//   - sbdac.c     *
//   - sbfm.c      *
//   - sb.h
// 02
//   - sb.c        *
//   - sb.h        *
int main(int argc, char **argv)
{
  int stopProg=FALSE;
//  WINDOW *statusWin;
  WINDOW *commandWin;
//  WINDOW *helpWin;
  int key;     // user input
  char buf[BUFLEN];
  int res;     // result of send command
  int n=0;       // # chars in buf
  int i;       // transmitting buffer

  GetBlasterEnv();
//  statusWin=  InitWindow(1,1,80,2,FALSE);
  commandWin= InitWindow(1,1,80,25,TRUE);
//  helpWin=    InitWindow(41,3,80,25,FALSE);

// 3 windows:
//    - top 2 lines
//    - rest of screen split in 2, left & right
//
  SetWindow(commandWin);
  cprintf("Blaster Port=%04x irq=%x dma=%d\r\n", SbAddr, SbIRQ, SbDMA);
  while (!stopProg)
  {
    if (kbhit())
    {
      key=getkey();
      switch(key)
      {
        case 'o':
          cprintf("enter string: ");
          n=gethex(buf,BUFLEN);
          cprintf("\r\n");
//-----fall through---------!!!!!
        case ' ':
          res=OK;
          cprintf("DSP: ");
          for (i=0 ; i<n && dsp_cmd(buf[i])==OK ; i++)
            cprintf(" %02x", buf[i]&0xff);

          cprintf("  : %d bytes not sent to DSP\r\n", n-i);
          print_status();
          break;
        case 0x0d:   // enter
          print_status();
          break;
        case 'i':
          print_input();
          break;
        case 'm':
          display_mixer();
          break;
        case 'r':
          outportb(DSP_RESET, 1);   // wait 3 usec
          waitusec(3);
          outportb(DSP_RESET, 0);
// after about 100 usec 0xaa will be returned at the dataport
          printf("DSP reset\r\n");
          print_status();
          break;
        case 0x1b:
          stopProg=TRUE;
          break;

      }
    }
  }
  return 0;
}
