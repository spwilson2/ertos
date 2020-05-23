//
// Referenced Documents:
// - Versatile Application Baseboard for ARM926EJ-S
//   - DUI0225D_versatile_application_baseboard_arm926ej_s_ug.pdf
//
// - PrimeCell™ Color LCD Controller(PL110) Revision: r1p2
//   - DDI0161.pdf

const BASE: usize = 0x10120000;

// offsets
const LCDUPBASE: isize = 0x10;
const LCDLPBASE: isize = 0x14;


// Allocated Divice-Memory Map
const MEMORY_TOP: usize = 0x08000000;
const VGA_RESERVATION_LEN: isize = 24 * 640 * 480;
const VGA_RESERVATION_START: usize = MEMORY_TOP - (VGA_RESERVATION_LEN as usize);

const SVG_RESERVATION_LEN: isize = 32 * 800 * 600;
const SVG_RESERVATION_START: usize = MEMORY_TOP - (SVG_RESERVATION_LEN as usize);



pub struct Pl110 {
  base: usize,
}

impl Pl110 {

  //pub fn power_on(&mut self) {
  //  // 1. Vdd is simultaneously applied to the SoC that contains the CLCDC and
  //  //    panel display driver logic. The signals CLLP, CLCP, CLFP, CLAC,
  //  //    CLD[23:0], and CLLE are held LOW.
  //  //
  //  // 2. When Vdd is stabilized, a 1 is written to the LcdEn bit in the
  //  //    LCDControl Register. This puts the signals CLLP, CLCP, CLFP, CLAC,
  //  //    and CLLE into their active states but the CLD[23:0] signals remain
  //  //    LOW.
  //  //
  //  // 3. When the signals in Step 2 have stabilized, where appropriate, the
  //  //    contrast voltage Vee (this is not controlled or supplied by the
  //  //    CLCDC) is then applied
  //  //
  //  // 4. You can use a software timer routine, if required, to provide the
  //  //    minimum display specific delay time between application of the control
  //  //    signals and power to the panel display. On completion of the software
  //  //    timer routine, power is applied to the panel by writing a 1 to the
  //  //    LcdPwr bit within the LcdControl Register which, in turn, sets the
  //  //    CLPOWER signal HIGH and puts the CLD[23:0] signals into their active
  //  //    state. The CLPOWER signal is expected to be used to gate the power to
  //  //    the LCD panel
  //  //
  //}

  fn init(&mut self) {
    // TODO Set the device memory map addresses
    // LCDUPBASE and LCDLPBASE

  }

  // TODO Set the 
  // TODO Allow switching modes
  // TODO Allow writing chars

}
