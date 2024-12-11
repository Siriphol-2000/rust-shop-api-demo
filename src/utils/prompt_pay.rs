use crc::{Algorithm, Crc};
use image::{ Luma};
use qrcode::{EcLevel, QrCode};

pub struct PromptPayUtils;

impl PromptPayUtils {
    /// Generate a PromptPay QR Code image and save it to the specified path
    ///
    /// # Arguments
    /// * `phone_number` - The Thai mobile number (10 digits, starting with 0)
    /// * `amount` - The transaction amount in Thai Baht
    /// * `output_path` - File path to save the QR code image
    pub fn generate_qr(
        phone_number: String,
        amount: f64,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Generate the payload string
        let payload = Self::generate_payload(phone_number, amount)?;

        // Generate the QR Code using the payload
        let code = QrCode::with_error_correction_level(payload.as_bytes(), EcLevel::L)?;

        // Render the QR Code as an image
        let image = code.render::<Luma<u8>>().build();
        image.save(output_path)?;

        println!("QR Code generated and saved at {}", output_path);

        Ok(())
    }

    /// Generate the PromptPay payload string
    fn generate_payload(phone_number: String, amount: f64) -> Result<String, String> {
        // Sanitize the phone number to ensure it meets PromptPay's requirements
        let sanitized_phone = Self::sanitize_phone_number(phone_number)?;

        // Convert the amount into satangs (1 Baht = 100 Satangs)
        let amount_satangs = (amount * 100.0).round() as u32;
        let formatted_amount = format!("{:08}", amount_satangs); // Pad to 8 digits

        // Construct the payload without the CRC
        let payload = format!(
            "00020101021129370016A00000067701011101130066{:0>9}5802TH53037645408{}6304",
            sanitized_phone, formatted_amount
        );

        // Calculate the CRC and append it to the payload
        let crc = Self::calculate_precise_crc(&payload);
        println!("This is payload: {}", payload);
        println!("This is return CRC: {}", crc);
        let final_payload = format!("{}{}", payload, crc);

        // Debugging: print the final payload
        println!("Generated Payload: {}", final_payload);

        Ok(final_payload)
    }

    /// Sanitize the phone number by ensuring it's valid for PromptPay
    fn sanitize_phone_number(phone_number: String) -> Result<String, String> {
        let sanitized = phone_number
            .trim()
            .replace(['-', '+'], "")
            .replace("66", "")
            .trim_start_matches('0') // Trim leading '0'
            .to_string();

        if sanitized.len() != 9 {
            return Err("Invalid phone number format".to_string());
        }

        Ok(sanitized)
    }

    /// Calculate the CRC-16 XMODEM checksum for the payload
    fn calculate_precise_crc(payload: &str) -> String {
        // Define the CRC-16 XMODEM algorithm
        const CRC_16_XMODEM: Crc<u16> = Crc::<u16>::new(&Algorithm {
            width: 16,
            poly: 0x1021,   // Polynomial for XMODEM
            init: 0xFFFF,   // Initial value
            refin: false,   // No reflection of input bits
            refout: false,  // No reflection of output bits
            xorout: 0x0000, // No XOR applied to the output
            check: 0x906E,  // Check value for validation
            residue: 0x0000,
        });

        // Calculate the CRC for the payload
        let mut digest = CRC_16_XMODEM.digest();
        digest.update(payload.as_bytes());
        let crc_value = digest.finalize();

        // Convert the CRC value to a 4-character hexadecimal string
        let formatted_crc = format!("{:04X}", crc_value);

        // Debugging outputs
        println!("Raw CRC Value: {}", crc_value);
        println!("Formatted CRC Value: {}", formatted_crc);

        formatted_crc
    }
}
