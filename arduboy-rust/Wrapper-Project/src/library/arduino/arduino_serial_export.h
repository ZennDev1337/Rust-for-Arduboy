

extern "C"
{
    void arduino_serial_begin(uint8_t serial)
    {
        Serial.begin(serial);
    }
    void arduino_serial_end()
    {
        Serial.end();
    }
    int arduino_serial_available()
    {
        return Serial.available();
    }
    int arduino_serial_read()
    {
        return Serial.read();
    }
    void arduino_serial_println_chars(const char *cstr)
    {
        Serial.println(cstr);
    }
    size_t arduino_serial_println_chars_progmem(const char *cstr)
    {
        return Serial.println(reinterpret_cast<const __FlashStringHelper *>(cstr));
    }
    size_t arduino_serial_println_char(char c)
    {
        return Serial.println(c);
    }
    size_t arduino_serial_println_int(int n, int base)
    {
        return Serial.println(n, base);
    }
    size_t arduino_serial_println_long(long n, int base)
    {
        return Serial.println(n, base);
    }
    size_t arduino_serial_println_unsigned_char(unsigned char n, int base)
    {
        return Serial.println(n, base);
    }
    size_t arduino_serial_println_unsigned_int(unsigned int n, int base)
    {
        return Serial.println(n, base);
    }
    size_t arduino_serial_println_unsigned_long(unsigned long n, int base)
    {
        return Serial.println(n, base);
    }
    void arduino_serial_print_chars(const char *cstr)
    {
        Serial.print(cstr);
    }
    size_t arduino_serial_print_chars_progmem(const char *cstr)
    {
        return Serial.print(reinterpret_cast<const __FlashStringHelper *>(cstr));
    }
    size_t arduino_serial_print_char(char c)
    {
        return Serial.print(c);
    }
    size_t arduino_serial_print_int(int n, int base)
    {
        return Serial.print(n, base);
    }
    size_t arduino_serial_print_long(long n, int base)
    {
        return Serial.print(n, base);
    }
    size_t arduino_serial_print_unsigned_char(unsigned char n, int base)
    {
        return Serial.print(n, base);
    }
    size_t arduino_serial_print_unsigned_int(unsigned int n, int base)
    {
        return Serial.print(n, base);
    }
    size_t arduino_serial_print_unsigned_long(unsigned long n, int base)
    {
        return Serial.print(n, base);
    }
}