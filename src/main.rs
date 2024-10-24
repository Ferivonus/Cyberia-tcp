use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    // I2P adresine TCP portu üzerinden bağlan
    let server_address = "rt42lgomkrottxd4il3uhasplte7mieiqi7qlbb5yu25fdxfqrna.b32.i2p:6667";

    // TCP bağlantısı aç
    let mut stream = match TcpStream::connect(server_address) {
        Ok(stream) => {
            println!("Bağlandı: {}", server_address);
            stream
        }
        Err(e) => {
            eprintln!("Bağlantı hatası: {}", e);
            return Err(e);
        }
    };

    // IRC sunucusuna 'NICK' ve 'USER' komutlarını gönder
    let nick = "NICK Ferivonus\r\n";
    let user = "USER Ferivonus 0 * :Ferivonus\r\n";

    // Mesajları TCP soketi üzerinden gönder
    stream.write_all(nick.as_bytes())?;
    stream.write_all(user.as_bytes())?;

    // Sunucudan gelen cevabı oku ve ekrana yaz
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    println!(
        "Sunucudan gelen cevap: {}",
        str::from_utf8(&buffer[..bytes_read]).unwrap_or("")
    );

    Ok(())
}
