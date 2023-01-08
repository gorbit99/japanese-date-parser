use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ERAS: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("大化", 645);
        m.insert("白雉", 650);
        m.insert("朱鳥", 686);
        m.insert("大宝", 701);
        m.insert("慶雲", 704);
        m.insert("和銅", 708);
        m.insert("霊亀", 715);
        m.insert("養老", 717);
        m.insert("神亀", 724);
        m.insert("天平", 729);
        m.insert("天平感宝", 749);
        m.insert("天平勝宝", 749);
        m.insert("天平宝字", 757);
        m.insert("天平神護", 765);
        m.insert("神護景雲", 767);
        m.insert("宝亀", 770);
        m.insert("天応", 781);
        m.insert("延暦", 782);
        m.insert("大同", 806);
        m.insert("弘仁", 810);
        m.insert("天長", 824);
        m.insert("承和", 834);
        m.insert("嘉祥", 848);
        m.insert("仁寿", 851);
        m.insert("斉衡", 854);
        m.insert("天安", 857);
        m.insert("貞観", 859);
        m.insert("元慶", 877);
        m.insert("仁和", 885);
        m.insert("寛平", 889);
        m.insert("昌泰", 898);
        m.insert("延喜", 901);
        m.insert("延長", 923);
        m.insert("承平", 931);
        m.insert("天慶", 938);
        m.insert("天暦", 947);
        m.insert("天徳", 957);
        m.insert("応和", 961);
        m.insert("康保", 964);
        m.insert("安和", 968);
        m.insert("天禄", 970);
        m.insert("天延", 973);
        m.insert("貞元", 976);
        m.insert("天元", 978);
        m.insert("永観", 983);
        m.insert("寛和", 985);
        m.insert("永延", 987);
        m.insert("永祚", 988);
        m.insert("正暦", 990);
        m.insert("長徳", 995);
        m.insert("長保", 999);
        m.insert("寛弘", 1004);
        m.insert("長和", 1012);
        m.insert("寛仁", 1017);
        m.insert("治安", 1021);
        m.insert("万寿", 1024);
        m.insert("長元", 1028);
        m.insert("長暦", 1037);
        m.insert("長久", 1040);
        m.insert("寛徳", 1044);
        m.insert("永承", 1046);
        m.insert("天喜", 1053);
        m.insert("康平", 1058);
        m.insert("治暦", 1065);
        m.insert("延久", 1069);
        m.insert("承保", 1074);
        m.insert("承暦", 1077);
        m.insert("永保", 1081);
        m.insert("応徳", 1084);
        m.insert("寛治", 1087);
        m.insert("嘉保", 1094);
        m.insert("永長", 1096);
        m.insert("承徳", 1097);
        m.insert("康和", 1099);
        m.insert("長治", 1104);
        m.insert("嘉承", 1106);
        m.insert("天仁", 1108);
        m.insert("天永", 1110);
        m.insert("永久", 1113);
        m.insert("保安", 1120);
        m.insert("天治", 1124);
        m.insert("大治", 1126);
        m.insert("天承", 1131);
        m.insert("長承", 1132);
        m.insert("保延", 1135);
        m.insert("永治", 1141);
        m.insert("康治", 1142);
        m.insert("天養", 1144);
        m.insert("久安", 1145);
        m.insert("仁平", 1151);
        m.insert("久寿", 1154);
        m.insert("保元", 1156);
        m.insert("平治", 1159);
        m.insert("永暦", 1160);
        m.insert("応保", 1161);
        m.insert("長寛", 1163);
        m.insert("永万", 1165);
        m.insert("仁安", 1166);
        m.insert("嘉応", 1169);
        m.insert("承安", 1171);
        m.insert("安元", 1175);
        m.insert("治承", 1177);
        m.insert("養和", 1181);
        m.insert("寿永", 1182);
        m.insert("元暦", 1184);
        m.insert("文治", 1185);
        m.insert("建久", 1190);
        m.insert("正治", 1199);
        m.insert("建仁", 1201);
        m.insert("元久", 1204);
        m.insert("建永", 1206);
        m.insert("承元", 1207);
        m.insert("建暦", 1211);
        m.insert("建保", 1213);
        m.insert("承久", 1219);
        m.insert("貞応", 1222);
        m.insert("元仁", 1224);
        m.insert("嘉禄", 1225);
        m.insert("安貞", 1227);
        m.insert("寛喜", 1229);
        m.insert("貞永", 1232);
        m.insert("天福", 1233);
        m.insert("文暦", 1234);
        m.insert("嘉禎", 1235);
        m.insert("暦仁", 1238);
        m.insert("延応", 1239);
        m.insert("仁治", 1240);
        m.insert("寛元", 1243);
        m.insert("宝治", 1247);
        m.insert("建長", 1249);
        m.insert("康元", 1256);
        m.insert("正嘉", 1257);
        m.insert("正元", 1259);
        m.insert("文応", 1260);
        m.insert("弘長", 1261);
        m.insert("文永", 1264);
        m.insert("建治", 1275);
        m.insert("弘安", 1278);
        m.insert("正応", 1288);
        m.insert("永仁", 1293);
        m.insert("正安", 1299);
        m.insert("乾元", 1302);
        m.insert("嘉元", 1303);
        m.insert("徳治", 1306);
        m.insert("延慶", 1308);
        m.insert("応長", 1311);
        m.insert("正和", 1312);
        m.insert("文保", 1317);
        m.insert("元応", 1319);
        m.insert("元亨", 1321);
        m.insert("正中", 1324);
        m.insert("嘉暦", 1326);
        m.insert("元徳", 1329);
        m.insert("元弘", 1331);
        m.insert("建武", 1334);
        m.insert("延元", 1336);
        m.insert("興国", 1340);
        m.insert("正平", 1346);
        m.insert("建徳", 1370);
        m.insert("文中", 1372);
        m.insert("天授", 1375);
        m.insert("弘和", 1381);
        m.insert("元中", 1384);
        m.insert("正慶", 1332);
        m.insert("暦応", 1338);
        m.insert("康永", 1342);
        m.insert("貞和", 1345);
        m.insert("観応", 1350);
        m.insert("文和", 1352);
        m.insert("延文", 1356);
        m.insert("康安", 1361);
        m.insert("貞治", 1362);
        m.insert("応安", 1368);
        m.insert("永和", 1375);
        m.insert("康暦", 1379);
        m.insert("永徳", 1381);
        m.insert("至徳", 1384);
        m.insert("嘉慶", 1387);
        m.insert("康応", 1389);
        m.insert("明徳", 1390);
        m.insert("応永", 1394);
        m.insert("正長", 1428);
        m.insert("永享", 1429);
        m.insert("嘉吉", 1441);
        m.insert("文安", 1444);
        m.insert("享徳", 1452);
        m.insert("康正", 1455);
        m.insert("長禄", 1457);
        m.insert("寛正", 1460);
        m.insert("文正", 1466);
        m.insert("応仁", 1467);
        m.insert("文明", 1469);
        m.insert("長享", 1487);
        m.insert("延徳", 1489);
        m.insert("明応", 1492);
        m.insert("文亀", 1501);
        m.insert("永正", 1504);
        m.insert("大永", 1521);
        m.insert("享禄", 1528);
        m.insert("天文", 1532);
        m.insert("弘治", 1555);
        m.insert("永禄", 1558);
        m.insert("元亀", 1570);
        m.insert("天正", 1573);
        m.insert("文禄", 1592);
        m.insert("慶長", 1596);
        m.insert("元和", 1615);
        m.insert("寛永", 1624);
        m.insert("正保", 1644);
        m.insert("慶安", 1648);
        m.insert("承応", 1652);
        m.insert("明暦", 1655);
        m.insert("万治", 1658);
        m.insert("寛文", 1661);
        m.insert("延宝", 1673);
        m.insert("天和", 1681);
        m.insert("貞享", 1684);
        m.insert("元禄", 1688);
        m.insert("宝永", 1704);
        m.insert("享保", 1716);
        m.insert("元文", 1736);
        m.insert("寛保", 1741);
        m.insert("延享", 1744);
        m.insert("寛延", 1748);
        m.insert("宝暦", 1751);
        m.insert("明和", 1764);
        m.insert("安永", 1772);
        m.insert("天明", 1781);
        m.insert("寛政", 1789);
        m.insert("享和", 1801);
        m.insert("文化", 1804);
        m.insert("文政", 1818);
        m.insert("天保", 1830);
        m.insert("弘化", 1844);
        m.insert("嘉永", 1848);
        m.insert("安政", 1854);
        m.insert("万延", 1860);
        m.insert("文久", 1861);
        m.insert("元治", 1864);
        m.insert("慶応", 1865);
        m.insert("明治", 1868);
        m.insert("大正", 1912);
        m.insert("昭和", 1926);
        m.insert("平成", 1989);
        m.insert("令和", 2019);
        m
    };
}

lazy_static! {
    pub static ref SHORTENED_ERAS: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('M', 1868);
        m.insert('T', 1912);
        m.insert('S', 1926);
        m.insert('H', 1989);
        m.insert('R', 2019);
        m.insert('明', 1868);
        m.insert('大', 1912);
        m.insert('昭', 1926);
        m.insert('平', 1989);
        m.insert('令', 2019);
        m
    };
}
