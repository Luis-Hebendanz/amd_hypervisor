use crate::interrupts::{
    default_diverging_handler, default_diverging_with_error_handler, default_handler,
    default_handler_with_error,
};
use x86::structures::idt::InterruptDescriptorTable;

pub fn init_default_handlers(idt: &mut InterruptDescriptorTable) {
    idt[0].set_handler_fn(default_handler::<0>);
    idt[1].set_handler_fn(default_handler::<1>);
    idt[2].set_handler_fn(default_handler::<2>);
    idt[3].set_handler_fn(default_handler::<3>);
    idt[4].set_handler_fn(default_handler::<4>);
    idt[5].set_handler_fn(default_handler::<5>);
    idt[6].set_handler_fn(default_handler::<6>);
    idt[7].set_handler_fn(default_handler::<7>);
    idt.double_fault
        .set_handler_fn(default_diverging_with_error_handler::<8>);

    idt[9].set_handler_fn(default_handler::<9>);
    idt.invalid_tss
        .set_handler_fn(default_handler_with_error::<10>);
    idt.segment_not_present
        .set_handler_fn(default_handler_with_error::<11>);
    idt.stack_segment_fault
        .set_handler_fn(default_handler_with_error::<12>);
    idt.general_protection_fault
        .set_handler_fn(default_handler_with_error::<13>);

    idt[16].set_handler_fn(default_handler::<16>);
    idt.alignment_check
        .set_handler_fn(default_handler_with_error::<17>);
    idt.machine_check
        .set_handler_fn(default_diverging_handler::<18>); // diverging
    idt[19].set_handler_fn(default_handler::<19>);
    idt[20].set_handler_fn(default_handler::<20>);
    idt.security_exception
        .set_handler_fn(default_handler_with_error::<30>);
    idt[32].set_handler_fn(default_handler::<32>);
    idt[33].set_handler_fn(default_handler::<33>);
    idt[34].set_handler_fn(default_handler::<34>);
    idt[35].set_handler_fn(default_handler::<35>);
    idt[36].set_handler_fn(default_handler::<36>);
    idt[37].set_handler_fn(default_handler::<37>);
    idt[38].set_handler_fn(default_handler::<38>);
    idt[39].set_handler_fn(default_handler::<39>);
    idt[40].set_handler_fn(default_handler::<40>);
    idt[41].set_handler_fn(default_handler::<41>);
    idt[42].set_handler_fn(default_handler::<42>);
    idt[43].set_handler_fn(default_handler::<43>);
    idt[44].set_handler_fn(default_handler::<44>);
    idt[45].set_handler_fn(default_handler::<45>);
    idt[46].set_handler_fn(default_handler::<46>);
    idt[47].set_handler_fn(default_handler::<47>);
    idt[48].set_handler_fn(default_handler::<48>);
    idt[49].set_handler_fn(default_handler::<49>);
    idt[50].set_handler_fn(default_handler::<50>);
    idt[51].set_handler_fn(default_handler::<51>);
    idt[52].set_handler_fn(default_handler::<52>);
    idt[53].set_handler_fn(default_handler::<53>);
    idt[54].set_handler_fn(default_handler::<54>);
    idt[55].set_handler_fn(default_handler::<55>);
    idt[56].set_handler_fn(default_handler::<56>);
    idt[57].set_handler_fn(default_handler::<57>);
    idt[58].set_handler_fn(default_handler::<58>);
    idt[59].set_handler_fn(default_handler::<59>);
    idt[60].set_handler_fn(default_handler::<60>);
    idt[61].set_handler_fn(default_handler::<61>);
    idt[62].set_handler_fn(default_handler::<62>);
    idt[63].set_handler_fn(default_handler::<63>);
    idt[64].set_handler_fn(default_handler::<64>);
    idt[65].set_handler_fn(default_handler::<65>);
    idt[66].set_handler_fn(default_handler::<66>);
    idt[67].set_handler_fn(default_handler::<67>);
    idt[68].set_handler_fn(default_handler::<68>);
    idt[69].set_handler_fn(default_handler::<69>);
    idt[70].set_handler_fn(default_handler::<70>);
    idt[71].set_handler_fn(default_handler::<71>);
    idt[72].set_handler_fn(default_handler::<72>);
    idt[73].set_handler_fn(default_handler::<73>);
    idt[74].set_handler_fn(default_handler::<74>);
    idt[75].set_handler_fn(default_handler::<75>);
    idt[76].set_handler_fn(default_handler::<76>);
    idt[77].set_handler_fn(default_handler::<77>);
    idt[78].set_handler_fn(default_handler::<78>);
    idt[79].set_handler_fn(default_handler::<79>);
    idt[80].set_handler_fn(default_handler::<80>);
    idt[81].set_handler_fn(default_handler::<81>);
    idt[82].set_handler_fn(default_handler::<82>);
    idt[83].set_handler_fn(default_handler::<83>);
    idt[84].set_handler_fn(default_handler::<84>);
    idt[85].set_handler_fn(default_handler::<85>);
    idt[86].set_handler_fn(default_handler::<86>);
    idt[87].set_handler_fn(default_handler::<87>);
    idt[88].set_handler_fn(default_handler::<88>);
    idt[89].set_handler_fn(default_handler::<89>);
    idt[90].set_handler_fn(default_handler::<90>);
    idt[91].set_handler_fn(default_handler::<91>);
    idt[92].set_handler_fn(default_handler::<92>);
    idt[93].set_handler_fn(default_handler::<93>);
    idt[94].set_handler_fn(default_handler::<94>);
    idt[95].set_handler_fn(default_handler::<95>);
    idt[96].set_handler_fn(default_handler::<96>);
    idt[97].set_handler_fn(default_handler::<97>);
    idt[98].set_handler_fn(default_handler::<98>);
    idt[99].set_handler_fn(default_handler::<99>);
    idt[100].set_handler_fn(default_handler::<100>);
    idt[101].set_handler_fn(default_handler::<101>);
    idt[102].set_handler_fn(default_handler::<102>);
    idt[103].set_handler_fn(default_handler::<103>);
    idt[104].set_handler_fn(default_handler::<104>);
    idt[105].set_handler_fn(default_handler::<105>);
    idt[106].set_handler_fn(default_handler::<106>);
    idt[107].set_handler_fn(default_handler::<107>);
    idt[108].set_handler_fn(default_handler::<108>);
    idt[109].set_handler_fn(default_handler::<109>);
    idt[110].set_handler_fn(default_handler::<110>);
    idt[111].set_handler_fn(default_handler::<111>);
    idt[112].set_handler_fn(default_handler::<112>);
    idt[113].set_handler_fn(default_handler::<113>);
    idt[114].set_handler_fn(default_handler::<114>);
    idt[115].set_handler_fn(default_handler::<115>);
    idt[116].set_handler_fn(default_handler::<116>);
    idt[117].set_handler_fn(default_handler::<117>);
    idt[118].set_handler_fn(default_handler::<118>);
    idt[119].set_handler_fn(default_handler::<119>);
    idt[120].set_handler_fn(default_handler::<120>);
    idt[121].set_handler_fn(default_handler::<121>);
    idt[122].set_handler_fn(default_handler::<122>);
    idt[123].set_handler_fn(default_handler::<123>);
    idt[124].set_handler_fn(default_handler::<124>);
    idt[125].set_handler_fn(default_handler::<125>);
    idt[126].set_handler_fn(default_handler::<126>);
    idt[127].set_handler_fn(default_handler::<127>);
    idt[128].set_handler_fn(default_handler::<128>);
    idt[129].set_handler_fn(default_handler::<129>);
    idt[130].set_handler_fn(default_handler::<130>);
    idt[131].set_handler_fn(default_handler::<131>);
    idt[132].set_handler_fn(default_handler::<132>);
    idt[133].set_handler_fn(default_handler::<133>);
    idt[134].set_handler_fn(default_handler::<134>);
    idt[135].set_handler_fn(default_handler::<135>);
    idt[136].set_handler_fn(default_handler::<136>);
    idt[137].set_handler_fn(default_handler::<137>);
    idt[138].set_handler_fn(default_handler::<138>);
    idt[139].set_handler_fn(default_handler::<139>);
    idt[140].set_handler_fn(default_handler::<140>);
    idt[141].set_handler_fn(default_handler::<141>);
    idt[142].set_handler_fn(default_handler::<142>);
    idt[143].set_handler_fn(default_handler::<143>);
    idt[144].set_handler_fn(default_handler::<144>);
    idt[145].set_handler_fn(default_handler::<145>);
    idt[146].set_handler_fn(default_handler::<146>);
    idt[147].set_handler_fn(default_handler::<147>);
    idt[148].set_handler_fn(default_handler::<148>);
    idt[149].set_handler_fn(default_handler::<149>);
    idt[150].set_handler_fn(default_handler::<150>);
    idt[151].set_handler_fn(default_handler::<151>);
    idt[152].set_handler_fn(default_handler::<152>);
    idt[153].set_handler_fn(default_handler::<153>);
    idt[154].set_handler_fn(default_handler::<154>);
    idt[155].set_handler_fn(default_handler::<155>);
    idt[156].set_handler_fn(default_handler::<156>);
    idt[157].set_handler_fn(default_handler::<157>);
    idt[158].set_handler_fn(default_handler::<158>);
    idt[159].set_handler_fn(default_handler::<159>);
    idt[160].set_handler_fn(default_handler::<160>);
    idt[161].set_handler_fn(default_handler::<161>);
    idt[162].set_handler_fn(default_handler::<162>);
    idt[163].set_handler_fn(default_handler::<163>);
    idt[164].set_handler_fn(default_handler::<164>);
    idt[165].set_handler_fn(default_handler::<165>);
    idt[166].set_handler_fn(default_handler::<166>);
    idt[167].set_handler_fn(default_handler::<167>);
    idt[168].set_handler_fn(default_handler::<168>);
    idt[169].set_handler_fn(default_handler::<169>);
    idt[170].set_handler_fn(default_handler::<170>);
    idt[171].set_handler_fn(default_handler::<171>);
    idt[172].set_handler_fn(default_handler::<172>);
    idt[173].set_handler_fn(default_handler::<173>);
    idt[174].set_handler_fn(default_handler::<174>);
    idt[175].set_handler_fn(default_handler::<175>);
    idt[176].set_handler_fn(default_handler::<176>);
    idt[177].set_handler_fn(default_handler::<177>);
    idt[178].set_handler_fn(default_handler::<178>);
    idt[179].set_handler_fn(default_handler::<179>);
    idt[180].set_handler_fn(default_handler::<180>);
    idt[181].set_handler_fn(default_handler::<181>);
    idt[182].set_handler_fn(default_handler::<182>);
    idt[183].set_handler_fn(default_handler::<183>);
    idt[184].set_handler_fn(default_handler::<184>);
    idt[185].set_handler_fn(default_handler::<185>);
    idt[186].set_handler_fn(default_handler::<186>);
    idt[187].set_handler_fn(default_handler::<187>);
    idt[188].set_handler_fn(default_handler::<188>);
    idt[189].set_handler_fn(default_handler::<189>);
    idt[190].set_handler_fn(default_handler::<190>);
    idt[191].set_handler_fn(default_handler::<191>);
    idt[192].set_handler_fn(default_handler::<192>);
    idt[193].set_handler_fn(default_handler::<193>);
    idt[194].set_handler_fn(default_handler::<194>);
    idt[195].set_handler_fn(default_handler::<195>);
    idt[196].set_handler_fn(default_handler::<196>);
    idt[197].set_handler_fn(default_handler::<197>);
    idt[198].set_handler_fn(default_handler::<198>);
    idt[199].set_handler_fn(default_handler::<199>);
    idt[200].set_handler_fn(default_handler::<200>);
    idt[201].set_handler_fn(default_handler::<201>);
    idt[202].set_handler_fn(default_handler::<202>);
    idt[203].set_handler_fn(default_handler::<203>);
    idt[204].set_handler_fn(default_handler::<204>);
    idt[205].set_handler_fn(default_handler::<205>);
    idt[206].set_handler_fn(default_handler::<206>);
    idt[207].set_handler_fn(default_handler::<207>);
    idt[208].set_handler_fn(default_handler::<208>);
    idt[209].set_handler_fn(default_handler::<209>);
    idt[210].set_handler_fn(default_handler::<210>);
    idt[211].set_handler_fn(default_handler::<211>);
    idt[212].set_handler_fn(default_handler::<212>);
    idt[213].set_handler_fn(default_handler::<213>);
    idt[214].set_handler_fn(default_handler::<214>);
    idt[215].set_handler_fn(default_handler::<215>);
    idt[216].set_handler_fn(default_handler::<216>);
    idt[217].set_handler_fn(default_handler::<217>);
    idt[218].set_handler_fn(default_handler::<218>);
    idt[219].set_handler_fn(default_handler::<219>);
    idt[220].set_handler_fn(default_handler::<220>);
    idt[221].set_handler_fn(default_handler::<221>);
    idt[222].set_handler_fn(default_handler::<222>);
    idt[223].set_handler_fn(default_handler::<223>);
    idt[224].set_handler_fn(default_handler::<224>);
    idt[225].set_handler_fn(default_handler::<225>);
    idt[226].set_handler_fn(default_handler::<226>);
    idt[227].set_handler_fn(default_handler::<227>);
    idt[228].set_handler_fn(default_handler::<228>);
    idt[229].set_handler_fn(default_handler::<229>);
    idt[230].set_handler_fn(default_handler::<230>);
    idt[231].set_handler_fn(default_handler::<231>);
    idt[232].set_handler_fn(default_handler::<232>);
    idt[233].set_handler_fn(default_handler::<233>);
    idt[234].set_handler_fn(default_handler::<234>);
    idt[235].set_handler_fn(default_handler::<235>);
    idt[236].set_handler_fn(default_handler::<236>);
    idt[237].set_handler_fn(default_handler::<237>);
    idt[238].set_handler_fn(default_handler::<238>);
    idt[239].set_handler_fn(default_handler::<239>);
    idt[240].set_handler_fn(default_handler::<240>);
    idt[241].set_handler_fn(default_handler::<241>);
    idt[242].set_handler_fn(default_handler::<242>);
    idt[243].set_handler_fn(default_handler::<243>);
    idt[244].set_handler_fn(default_handler::<244>);
    idt[245].set_handler_fn(default_handler::<245>);
    idt[246].set_handler_fn(default_handler::<246>);
    idt[247].set_handler_fn(default_handler::<247>);
    idt[248].set_handler_fn(default_handler::<248>);
    idt[249].set_handler_fn(default_handler::<249>);
    idt[250].set_handler_fn(default_handler::<250>);
    idt[251].set_handler_fn(default_handler::<251>);
    idt[252].set_handler_fn(default_handler::<252>);
    idt[253].set_handler_fn(default_handler::<253>);
    idt[254].set_handler_fn(default_handler::<254>);
    idt[255].set_handler_fn(default_handler::<255>);
}
