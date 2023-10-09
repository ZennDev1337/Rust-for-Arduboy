#pragma once

extern "C"
{
    void arduboyfx_begin(void)
    {
        FX::begin();
    }
    void arduboyfx_begin_data(uint16_t datapage)
    {
        FX::begin(datapage);
    }
    void arduboyfx_begin_data_save(uint16_t datapage,uint16_t savepage )
    {
        FX::begin(datapage,savepage);
    }
    void arduboyfx_display()
    {
        FX::display();
    }
    void arduboyfx_display_clear()
    {
        FX::display(true);
    }
    void arduboyfx_draw_bitmap(int16_t x,int16_t y,uint24_t address,uint8_t frame,uint8_t mode )
    {
         FX::drawBitmap(x,y,address,frame,mode);
    }
    void arduboyfx_read_data_array(uint24_t address,uint8_t index,uint8_t offset,uint8_t elementSize,uint8_t * buffer,size_t length )
    {
         FX::readDataArray(address,index,offset,elementSize,buffer,length);
    }
    void arduboyfx_set_frame(uint24_t frame,uint8_t repeat )
    {
         FX::setFrame(frame,repeat);
    }
    uint24_t arduboyfx_draw_frame(uint24_t address)
    {
         return FX::drawFrame(address);
    }
    void arduboyfx_set_cursor(int16_t x,int16_t y)
    {
         FX::setCursor(x,y);
    }
    void arduboyfx_draw_string_fx(uint24_t address)
    {
         FX::drawString(address);
    }
    void arduboyfx_draw_string_buffer(const uint8_t * buffer )
    {
         FX::drawString(buffer);
    }
    void arduboyfx_draw_string(const char *str)
    {
         FX::drawString(str);
    }
    void arduboyfx_set_cursor_x(int16_t x)
    {
         FX::setCursorX(x);
    }
    void arduboyfx_set_cursor_y(int16_t y)
    {
         FX::setCursorY(y);
    }
    void arduboyfx_set_font(uint24_t address, uint8_t mode)
    {
        FX::setFont(address,mode);
    }
    void arduboyfx_set_font_mode(uint8_t mode)
    {
        FX::setFontMode(mode);
    }
    void arduboyfx_set_cursor_range(int16_t left,int16_t wrap)
    {
        FX::setCursorRange(left, wrap);
    }
    void arduboyfx_draw_number_i16(int16_t n, int8_t digits)
    {
        FX::drawNumber(n,digits);
    }
    void arduboyfx_draw_number_i32(int32_t n, int8_t digits)
    {
        FX::drawNumber(n,digits);
    }
    void arduboyfx_draw_number_u16(uint16_t n, int8_t digits)
    {
        FX::drawNumber(n,digits);
    }
    void arduboyfx_draw_number_u32(uint32_t n, int8_t digits)
    {
        FX::drawNumber(n,digits);
    }
    void arduboyfx_draw_char(uint8_t c)
    {
        FX::drawChar(c);
    }
    uint8_t arduboyfx_load_game_state(uint8_t *gameState,size_t size)
    {
        return FX::loadGameState(gameState, size);
    }
    void arduboyfx_save_game_state(uint8_t *gameState,size_t size)
    {
        FX::saveGameState(gameState, size);
    }
}