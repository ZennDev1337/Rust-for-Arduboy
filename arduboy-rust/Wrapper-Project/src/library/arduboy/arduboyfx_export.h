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
    void arduboyfx_draw_string(uint24_t address)
    {
         FX::drawString(address);
    }
    void arduboyfx_draw_string_buffer(const uint8_t * buffer )
    {
         FX::drawString(buffer);
    }
    void arduboyfx_set_cursor(int16_t x,int16_t y)
    {
         return FX::setCursor(x,y);
    }
}