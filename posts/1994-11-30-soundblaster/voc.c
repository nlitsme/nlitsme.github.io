// voc file format:

typedef unsigned int word;
typedef unsigned char byte;
typedef unsigned char bool;

struct VOCblkVoicedata {   // block type BLK_VOICE_DATA
  byte sampleRate;   // 256-1000000/rate
  byte packingMethod;    //
  byte data[1];
};
#define VOC_PACK_8BIT_RAW   0
#define VOC_PACK_4BIT       1
#define VOC_PACK_2_6BIT     2
#define VOC_PACK_2BIT       3

struct VOCblkSilence {  // block type BLK_SILENCE
  word period;         // unit: sampling cycle,  period 0 = 1 cycle
  byte sampleRate;
};

struct VOCdataBlock {
  union {
    byte type;
    long length;   // bit31-24 are used for type
  } hdr;
  union {
                                         // BLK_TERMINATOR
    struct VOCblkVoiceData voice;        // BLK_VOICE_DATA
    byte                   voicedata[1]; // BLK_VOICE_DATA_CONT
    struct VOCblkSilence   silence;      // BLK_SILENCE
    word                   marker;       // BLK_MARKER
    char                   asciiz[1];    // BLK_ASCII_TEXT
    word                   repeatCount;  // BLK_REPEAT_LOOP
                                         // BLK_END_REPEAT
  } data;
};

struct VOCvocFile {
  char description[14];
  word dataStart;
  word version;
  word check;     // 0x1234+ ~version
  struct VOCdataBlock data[1];
};
// dataBlock types
#define BLK_TERMINATOR      0    // no data
#define BLK_VOICE_DATA      1    // struct VOCblkVoiceData
#define BLK_VOICE_DATA_CONT 2    // plain voice data
#define BLK_SILENCE         3    // struct VOCblkSilence
#define BLK_MARKER          4    // struct VOCblkMarker
#define BLK_ASCII_TEXT      5    // ASCIIZ string
#define BLK_REPEAT_LOOP     6    // struct VOCblkRepeat
#define BLK_END_REPEAT      7    // no data

// CMF format
struct CMFHeaderBlock {
  char fileID[4];          // "CTMF"
  word FileFormatVersion;
  word ofsInstrumentBlock; // offset
  word ofsMusicBlock;      // offset
  word ticksPQNote;        // ticks per quaternote
  word ticksPSecond;
// offsets to ASCIIZ description strings
  word ofsMusicTitle;      // 0 == none
  word ofsComposerName;
  word ofsRemarks;

  bool channelInUse[16];
  word nInstruments;
  word basicTempo;
  char descriptions[1];
};

// InstrumentBlock
//   - nInstruments 16 byte blocks : register set images
//   - identical to SBI format
// MusicBlock
//   - SMF (midi) format
//   - single track
//   - multi channel, max 16
//   - polyphonic
//
// smf: 3 types of events
//   - midi
//   - system exclusive
//   - meta events
//
// midi events:
//   - 0x66, data 0x01 - 0x7f   marker
//   - 0x67, data 0:melody mode,  1:rhythm mode
//      rhytm: last 5 channels=bass, snare, tomtom, topcymbal, highhat
// event is preceeded by a delta time:
//    the high bit of every byte signifies: more bytes follow
//    -> 7 bits of info per byte
//    byte order : hilo

struct instrumentDef {
  byte modSound;     // modulator sound characteristic
  byte carSound;     // carrier sound characteristic
    // .7  pitch vibrato (AM)
    // .6  amplitude vibrato (VIB)
    // .5  sustaining sound (EG-TYP)
    // .4  envelop scaling  (KSR)
    // .3210 frequency multiplier

  byte modScaling_level;
  byte carrierScaling_level;
    // .76  level scaling (KSL)
    // .5-0 output level (TL)

  byte modAttack_decay;
  byte carrierAttack_decay;
    // .7654  attack rate  (AR)
    // .3210  decay rate   (DR)

  byte modSustain_releas;
  byte carrierSustain_releas;
    // .7654  sustain level (SL)
    // .3210  release rate  (RR)

  byte modWaveSelect;
  byte carrierWaveSelect;
    // .7-2   reserved ==0
    // .10    wave select   (WS)

  byte feedback_connect;
    // .7654  reserved ==0
    // .321   modulator feedback  (FB)
    // .0     connection

  byte reserved[5];
}

// SBI format:
struct SBIHeader {
  char fileID[4];   // "SBI\032"
  char instrumentName[32];
  struct instrumentDef instrument;
};

// IBK  (instrument bank)
struct IBKheader {
  char fileID[4];  // "IBK\032"
  struct instrumentDef instruments[128];
  char instrumentNames[128][9];
};
