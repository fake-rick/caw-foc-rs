#![allow(dead_code)]

use defmt::*;

use embassy_stm32::{
    gpio::{Level, Output, Speed},
    mode::Blocking,
    spi::{self, Spi},
    time::Hertz,
};
use embassy_time::Timer;

use crate::{
    tasks::messages::{Commands, USART_WRITE_SIGNAL},
    Spi3Resources,
};

/// Registers
const FSR1: u16 = 0x0; // Fault Status Register 1
const FSR2: u16 = 0x1; // Fault Status Register 2
const DCR: u16 = 0x2; // Drive Control Register
const HSR: u16 = 0x3; // Gate Drive HS Register
const LSR: u16 = 0x4; // Gate Drive LS Register
const OCPCR: u16 = 0x5; // OCP Control Register
const CSACR: u16 = 0x6; // CSA Control Register

/// Drive Control Fields
const DIS_CPUV_EN: u16 = 0x0; // Charge pump UVLO fault
const DIS_CPUV_DIS: u16 = 0x1;
const DIS_GDF_EN: u16 = 0x0; // Gate drive fauilt
const DIS_GDF_DIS: u16 = 0x1;
const OTW_REP_EN: u16 = 0x1; // Over temp warning reported on nFAULT/FAULT bit
const OTW_REP_DIS: u16 = 0x0;
const PWM_MODE_6X: u16 = 0x0; // PWM Input Modes
const PWM_MODE_3X: u16 = 0x1;
const PWM_MODE_1X: u16 = 0x2;
const PWM_MODE_IND: u16 = 0x3;
const PWM_1X_COM_SYNC: u16 = 0x0; // 1x PWM Mode synchronou rectification
const PWM_1X_COM_ASYNC: u16 = 0x1;
const PWM_1X_DIR_0: u16 = 0x0; // In 1x PWM mode this bit is ORed with the INHC (DIR) input
const PWM_1X_DIR_1: u16 = 0x1;

/// Gate Drive HS Fields
const LOCK_ON: u16 = 0x6;
const LOCK_OFF: u16 = 0x3;
const IDRIVEP_HS_10MA: u16 = 0x0; // Gate drive high side turn on current
const IDRIVEP_HS_30MA: u16 = 0x1;
const IDRIVEP_HS_60MA: u16 = 0x2;
const IDRIVEP_HS_80MA: u16 = 0x3;
const IDRIVEP_HS_120MA: u16 = 0x4;
const IDRIVEP_HS_140MA: u16 = 0x5;
const IDRIVEP_HS_170MA: u16 = 0x6;
const IDRIVEP_HS_190MA: u16 = 0x7;
const IDRIVEP_HS_260MA: u16 = 0x8;
const IDRIVEP_HS_330MA: u16 = 0x9;
const IDRIVEP_HS_370MA: u16 = 0xA;
const IDRIVEP_HS_440MA: u16 = 0xB;
const IDRIVEP_HS_570MA: u16 = 0xC;
const IDRIVEP_HS_680MA: u16 = 0xD;
const IDRIVEP_HS_820MA: u16 = 0xE;
const IDRIVEP_HS_1000MA: u16 = 0xF;
const IDRIVEN_HS_20MA: u16 = 0x0; // High side turn off current
const IDRIVEN_HS_60MA: u16 = 0x1;
const IDRIVEN_HS_120MA: u16 = 0x2;
const IDRIVEN_HS_160MA: u16 = 0x3;
const IDRIVEN_HS_240MA: u16 = 0x4;
const IDRIVEN_HS_280MA: u16 = 0x5;
const IDRIVEN_HS_340MA: u16 = 0x6;
const IDRIVEN_HS_380MA: u16 = 0x7;
const IDRIVEN_HS_520MA: u16 = 0x8;
const IDRIVEN_HS_660MA: u16 = 0x9;
const IDRIVEN_HS_740MA: u16 = 0xA;
const IDRIVEN_HS_880MA: u16 = 0xB;
const IDRIVEN_HS_1140MA: u16 = 0xC;
const IDRIVEN_HS_1360MA: u16 = 0xD;
const IDRIVEN_HS_1640MA: u16 = 0xE;
const IDRIVEN_HS_2000MA: u16 = 0xF;

/// Gate Drive LS Fields
const TDRIVE_500NS: u16 = 0x0; // Peak gate-current drive time
const TDRIVE_1000NS: u16 = 0x1;
const TDRIVE_2000NS: u16 = 0x2;
const TDRIVE_4000NS: u16 = 0x3;
const IDRIVEP_LS_10MA: u16 = 0x0; // Gate drive high side turn on current
const IDRIVEP_LS_30MA: u16 = 0x1;
const IDRIVEP_LS_60MA: u16 = 0x2;
const IDRIVEP_LS_80MA: u16 = 0x3;
const IDRIVEP_LS_120MA: u16 = 0x4;
const IDRIVEP_LS_140MA: u16 = 0x5;
const IDRIVEP_LS_170MA: u16 = 0x6;
const IDRIVEP_LS_190MA: u16 = 0x7;
const IDRIVEP_LS_260MA: u16 = 0x8;
const IDRIVEP_LS_330MA: u16 = 0x9;
const IDRIVEP_LS_370MA: u16 = 0xA;
const IDRIVEP_LS_440MA: u16 = 0xB;
const IDRIVEP_LS_570MA: u16 = 0xC;
const IDRIVEP_LS_680MA: u16 = 0xD;
const IDRIVEP_LS_820MA: u16 = 0xE;
const IDRIVEP_LS_1000MA: u16 = 0xF;
const IDRIVEN_LS_20MA: u16 = 0x0; // High side turn off current
const IDRIVEN_LS_60MA: u16 = 0x1;
const IDRIVEN_LS_120MA: u16 = 0x2;
const IDRIVEN_LS_160MA: u16 = 0x3;
const IDRIVEN_LS_240MA: u16 = 0x4;
const IDRIVEN_LS_280MA: u16 = 0x5;
const IDRIVEN_LS_340MA: u16 = 0x6;
const IDRIVEN_LS_380MA: u16 = 0x7;
const IDRIVEN_LS_520MA: u16 = 0x8;
const IDRIVEN_LS_660MA: u16 = 0x9;
const IDRIVEN_LS_740MA: u16 = 0xA;
const IDRIVEN_LS_880MA: u16 = 0xB;
const IDRIVEN_LS_1140MA: u16 = 0xC;
const IDRIVEN_LS_1360MA: u16 = 0xD;
const IDRIVEN_LS_1640MA: u16 = 0xE;
const IDRIVEN_LS_2000MA: u16 = 0xF;

/// OCP Control Fields
const TRETRY_4MS: u16 = 0x0; // VDS OCP and SEN OCP retry time
const TRETRY_50US: u16 = 0x1;
const DEADTIME_50NS: u16 = 0x0; // Deadtime
const DEADTIME_100NS: u16 = 0x1;
const DEADTIME_200NS: u16 = 0x2;
const DEADTIME_400NS: u16 = 0x3;
const OCP_LATCH: u16 = 0x0; // OCP Mode
const OCP_RETRY: u16 = 0x1;
const OCP_REPORT: u16 = 0x2;
const OCP_NONE: u16 = 0x3;
const OCP_DEG_2US: u16 = 0x0; // OCP Deglitch Time
const OCP_DEG_4US: u16 = 0x1;
const OCP_DEG_6US: u16 = 0x2;
const OCP_DEG_8US: u16 = 0x3;
const VDS_LVL_0_06: u16 = 0x0;
const VDS_LVL_0_13: u16 = 0x1;
const VDS_LVL_0_2: u16 = 0x2;
const VDS_LVL_0_26: u16 = 0x3;
const VDS_LVL_0_31: u16 = 0x4;
const VDS_LVL_0_45: u16 = 0x5;
const VDS_LVL_0_53: u16 = 0x6;
const VDS_LVL_0_6: u16 = 0x7;
const VDS_LVL_0_68: u16 = 0x8;
const VDS_LVL_0_75: u16 = 0x9;
const VDS_LVL_0_94: u16 = 0xA;
const VDS_LVL_1_13: u16 = 0xB;
const VDS_LVL_1_3: u16 = 0xC;
const VDS_LVL_1_5: u16 = 0xD;
const VDS_LVL_1_7: u16 = 0xE;
const VDS_LVL_1_88: u16 = 0xF;

/// CSA Control Fields
const CSA_FET_SP: u16 = 0x0; // Current sense amplifier positive input
const CSA_FET_SH: u16 = 0x1;
const VREF_DIV_1: u16 = 0x0; // Amplifier reference voltage is VREV/1
const VREF_DIV_2: u16 = 0x1; // Amplifier reference voltage is VREV/2
const CSA_GAIN_5: u16 = 0x0; // Current sensor gain
const CSA_GAIN_10: u16 = 0x1;
const CSA_GAIN_20: u16 = 0x2;
const CSA_GAIN_40: u16 = 0x3;
const DIS_SEN_EN: u16 = 0x0; // Overcurrent Fault
const DIS_SEN_DIS: u16 = 0x1;
const SEN_LVL_0_25: u16 = 0x0; // Sense OCP voltage level
const SEN_LVL_0_5: u16 = 0x1;
const SEN_LVL_0_75: u16 = 0x2;
const SEN_LVL_1_0: u16 = 0x3;

pub struct DRV8232RS<'a> {
    cs: Output<'a>,
    spi: Spi<'a, Blocking>,
}

impl<'a> DRV8232RS<'a> {
    pub fn new(r: Spi3Resources) -> DRV8232RS<'a> {
        let cs = Output::new(r.cs, Level::High, Speed::Low);
        let mut config = spi::Config::default();
        config.frequency = Hertz(5_000_000);
        config.bit_order = spi::BitOrder::MsbFirst;
        config.mode = spi::MODE_0;
        let spi = spi::Spi::new_blocking(r.spi, r.sck, r.mosi, r.miso, config);
        DRV8232RS { cs, spi }
    }

    async fn write(&mut self, val: u16) -> u16 {
        self.cs.set_low();
        Timer::after_micros(10).await;
        let rx_data = 0u16;
        if let Err(err) = self
            .spi
            .blocking_transfer(&mut rx_data.to_le_bytes(), &val.to_le_bytes())
        {
            error!("{:?}", err);
        }
        self.cs.set_high();
        rx_data
    }

    pub async fn read_fsr1(&mut self) -> u16 {
        let val = (1u16 << 15) | FSR1;
        self.write(val).await
    }

    pub async fn read_fsr2(&mut self) -> u16 {
        let val = (1u16 << 15) | FSR2;
        self.write(val).await
    }

    pub async fn read_register(&mut self, reg: u16) -> u16 {
        self.write((1 << 15) | (reg << 11)).await
    }

    pub async fn write_register(&mut self, reg: u16, val: u16) {
        self.write((reg << 11) | val).await;
    }

    pub async fn write_dcr(
        &mut self,
        dis_cpuv: u16,
        dis_gdf: u16,
        otw_rep: u16,
        pwm_mode: u16,
        pwm_com: u16,
        pwm_dir: u16,
        coast: u16,
        brake: u16,
        clr_flt: u16,
    ) {
        let val = (DCR << 11)
            | (dis_cpuv << 9)
            | (dis_gdf << 8)
            | (otw_rep << 7)
            | (pwm_mode << 5)
            | (pwm_com << 4)
            | (pwm_dir << 3)
            | (coast << 2)
            | (brake << 1)
            | clr_flt;
        self.write(val).await;
    }

    pub async fn write_hsr(&mut self, lock: u16, idrivep_hs: u16, idriven_hs: u16) {
        let val = (HSR << 11) | (lock << 8) | (idrivep_hs << 4) | idriven_hs;
        self.write(val).await;
    }

    pub async fn write_lsr(&mut self, cbc: u16, tdrive: u16, idrivep_ls: u16, idriven_ls: u16) {
        let val = (LSR << 11) | (cbc << 10) | (tdrive << 8) | (idrivep_ls << 4) | idriven_ls;
        self.write(val).await;
    }

    pub async fn write_ocpcr(
        &mut self,
        tretry: u16,
        dead_time: u16,
        ocp_mode: u16,
        ocp_deg: u16,
        vds_lvl: u16,
    ) {
        let val = (OCPCR << 11)
            | (tretry << 10)
            | (dead_time << 8)
            | (ocp_mode << 6)
            | (ocp_deg << 4)
            | vds_lvl;
        self.write(val).await;
    }

    pub async fn write_csacr(
        &mut self,
        csa_fet: u16,
        vref_div: u16,
        ls_ref: u16,
        csa_gain: u16,
        dis_sen: u16,
        csa_cal_a: u16,
        csa_cal_b: u16,
        csa_cal_c: u16,
        sen_lvl: u16,
    ) {
        let val = (CSACR << 11)
            | (csa_fet << 10)
            | (vref_div << 9)
            | (ls_ref << 8)
            | (csa_gain << 6)
            | (dis_sen << 5)
            | (csa_cal_a << 4)
            | (csa_cal_b << 3)
            | (csa_cal_c << 2)
            | sen_lvl;
        self.write(val).await;
    }

    pub async fn print_faults(&mut self) {
        let val1 = self.read_fsr1().await;
        Timer::after_micros(10).await;
        let val2 = self.read_fsr2().await;
        Timer::after_micros(10).await;

        if val1 & (1 << 10) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("FAULT".as_bytes()));
        }
        if val1 & (1 << 9) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_OCP".as_bytes()));
        }
        if val1 & (1 << 8) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("GDF".as_bytes()));
        }
        if val1 & (1 << 7) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("UVLO".as_bytes()));
        }
        if val1 & (1 << 6) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("OTSD".as_bytes()));
        }
        if val1 & (1 << 5) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_HA".as_bytes()));
        }
        if val1 & (1 << 4) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_LA".as_bytes()));
        }
        if val1 & (1 << 3) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_HB".as_bytes()));
        }
        if val1 & (1 << 2) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_LB".as_bytes()));
        }
        if val1 & (1 << 1) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_HC".as_bytes()));
        }
        if val1 & 1 != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VDS_LC".as_bytes()));
        }

        if val2 & (1 << 10) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("SA_OC".as_bytes()));
        }
        if val2 & (1 << 9) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("SB_OC".as_bytes()));
        }
        if val2 & (1 << 8) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("SC_OC".as_bytes()));
        }
        if val2 & (1 << 7) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("OTW".as_bytes()));
        }
        if val2 & (1 << 6) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("CPUV".as_bytes()));
        }
        if val2 & (1 << 5) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VGS_HA".as_bytes()));
        }
        if val2 & (1 << 4) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VGS_LA".as_bytes()));
        }
        if val2 & (1 << 3) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VGS_HB".as_bytes()));
        }
        if val2 & (1 << 2) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VGS_LB".as_bytes()));
        }
        if val2 & (1 << 1) != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VGS_HC".as_bytes()));
        }
        if val2 & 1 != 0 {
            USART_WRITE_SIGNAL.signal(Commands::UsartTxBuf("VGS_LC".as_bytes()));
        }
    }

    pub async fn enable_gd(&mut self) {
        let val = (self.read_register(DCR).await) & (!(0x1u16 << 2));
        self.write_register(DCR, val).await;
    }

    pub async fn disable_gd(&mut self) {
        let val = (self.read_register(DCR).await) | (0x1u16 << 2);
        self.write_register(DCR, val).await;
    }

    pub async fn calibrate(&mut self) {
        let val = 0x1u16 << 4 + 0x1u16 << 3 + 0x1u16 << 2;
        self.write_register(CSACR, val).await;
    }
}
