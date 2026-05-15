use std::io;
use std::io::Write;

// ============================================================================
// STRUCT - Pemrograman Berbasis Objek
// ============================================================================

struct TankMonitoring {
    max_capacity: f32,      // Kapasitas maksimal (Liter)
    height: f32,            // Tinggi tangki (Meter)
    ll: f32,                // Low Level (Meter)
    hh: f32,                // High High (Meter)
    sensor_reading: f32,    // Pembacaan sensor (Meter)
}

impl TankMonitoring {
    // Konstruktor1
    fn new(max_capacity: f32, height: f32, ll: f32, hh: f32, sensor_reading: f32) -> Self {
        TankMonitoring {
            max_capacity,
            height,
            ll,
            hh,
            sensor_reading,
        }
    }
    
    // Konversi Meter ke Liter
    fn meter_to_liter(&self, meter: f32) -> f32 {
        (meter / self.height) * self.max_capacity
    }
    
    // Status Inlet Valve & Pump
    fn get_inlet_status(&self) -> (String, String) {
        if self.sensor_reading <= self.ll {
            ("OPEN".to_string(), "ON".to_string())
        } else {
            ("CLOSE".to_string(), "OFF".to_string())
        }
    }
    
    // Status Outlet Valve
    fn get_outlet_status(&self) -> String {
        if self.sensor_reading >= self.hh {
            "OPEN".to_string()
        } else {
            "CLOSE".to_string()
        }
    }
    
    // Status Kondisi Tangki
    fn get_condition(&self) -> String {
        if self.sensor_reading >= self.hh {
            "⛔ HIGH HIGH (HH) - ALARM!".to_string()
        } else if self.sensor_reading <= self.ll {
            "⛔ LOW LOW (LL) - ALARM!".to_string()
        } else {
            "✅ NORMAL".to_string()
        }
    }
    
    // Tampilkan Dashboard
    fn display(&self) {
        println!("\n{}", "=".repeat(60));
        println!("{:^60}", "📊 DASHBOARD TANGKI");
        println!("{}", "=".repeat(60));
        
        println!("\n📋 PARAMETER:");
        println!("  Kapasitas: {:.2} L | Tinggi: {:.2} m", self.max_capacity, self.height);
        println!("  LL: {:.2} m | HH: {:.2} m", self.ll, self.hh);
        
        let volume = self.meter_to_liter(self.sensor_reading);
        let percentage = (self.sensor_reading / self.height) * 100.0;
        
        println!("\n📈 DATA SAAT INI:");
        println!("  Level Sensor: {:.2} m", self.sensor_reading);
        println!("  Volume Air: {:.2} L", volume);
        println!("  Persentase: {:.1}%", percentage);
        
        println!("\n⚡ STATUS SISTEM:");
        println!("  Kondisi: {}", self.get_condition());
        let (inlet, pump) = self.get_inlet_status();
        let outlet = self.get_outlet_status();
        println!("  INLET: {} | PUMP: {} | OUTLET: {}", inlet, pump, outlet);
        
        println!("\n{}", "=".repeat(60));
    }
}

// ============================================================================
// FUNGSI HELPER
// ============================================================================

fn input_f32(prompt: &str) -> f32 {
    loop {
        print!("{}", prompt);
        let _ = Write::flush(&mut io::stdout());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<f32>() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("❌ Angka tidak valid!\n"),
        }
    }
}

// ============================================================================
// MAIN PROGRAM
// ============================================================================

fn main() {
    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║   SISTEM MONITORING LEVEL TANGKI - VERSI SEDERHANA     ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    loop {
        println!("\n📋 MENU:");
        println!("1. Input Parameter & Sensor");
        println!("2. Keluar");
        print!("Pilih (1-2): ");
        let _ = Write::flush(&mut io::stdout());
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                println!("\n{}", "-".repeat(50));
                println!("📥 INPUT DATA");
                println!("{}", "-".repeat(50));
                
                let max_cap = input_f32("Kapasitas tangki (Liter): ");
                let height = input_f32("Tinggi tangki (Meter): ");
                let ll = input_f32("Batas Low Level - LL (Meter): ");
                let hh = input_f32("Batas High High - HH (Meter): ");
                let sensor = input_f32("Pembacaan sensor saat ini (Meter): ");
                
                // Buat object tangki dengan input user
                let tank = TankMonitoring::new(max_cap, height, ll, hh, sensor);
                
                println!("\n✓ Data tersimpan!");
                tank.display();
            }
            
            "2" => {
                println!("\n👋 Selesai!\n");
                break;
            }
            
            _ => println!("❌ Pilihan tidak valid!\n"),
        }
    }
}