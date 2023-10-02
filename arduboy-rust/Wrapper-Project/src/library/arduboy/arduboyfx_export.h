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
}