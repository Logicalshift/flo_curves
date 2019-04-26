use flo_curves::*;
use flo_curves::debug::*;
use flo_curves::bezier::path::*;

use super::svg::*;

#[test]
fn remove_interior_points_1() {
    // Complicated curve found in FlowBetween that produces 0 points when interior points are removed
    // It appears this has three curves that converge on a single point, which generates two points in the output, 
    // which in turn produces a spurious edge, which prevents us from being able to follow the path all the way around.
    let curve = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(562.0692138671875, 669.944580078125))
        .curve_to((Coord2(562.0692138671875, 669.944580078125), Coord2(562.0692138671875, 669.944580078125)), Coord2(562.0692138671875, 669.944580078125))
        .curve_to((Coord2(562.4200439453125, 669.9562377929688), Coord2(562.6718139648438, 670.0160522460938)), Coord2(562.8291015625, 670.0160522460938))
        .curve_to((Coord2(562.7747802734375, 670.2525634765625), Coord2(563.1968383789063, 669.9431762695313)), Coord2(563.401611328125, 669.6962890625))
        .curve_to((Coord2(563.218505859375, 669.447021484375), Coord2(562.7468872070313, 668.9757690429688)), Coord2(562.6525268554688, 669.1633911132813))
        .curve_to((Coord2(562.3690185546875, 669.1181640625), Coord2(562.02490234375, 668.9761352539063)), Coord2(561.6610107421875, 668.9097290039063))
        .curve_to((Coord2(560.6327514648438, 668.3336181640625), Coord2(560.5078125, 668.680419921875)), Coord2(560.7962036132813, 668.913818359375))
        .curve_to((Coord2(560.7932739257813, 669.053955078125), Coord2(560.795166015625, 668.9855346679688)), Coord2(560.7711791992188, 669.1161499023438))
        .curve_to((Coord2(560.0169067382813, 670.2783813476563), Coord2(561.4442749023438, 669.9208984375)), Coord2(561.7700805664063, 668.3951416015625))
        .curve_to((Coord2(560.5978393554688, 668.2579956054688), Coord2(555.1843872070313, 665.8880615234375)), Coord2(552.6854248046875, 664.9908447265625))
        .curve_to((Coord2(552.62158203125, 664.9489135742188), Coord2(552.4188842773438, 664.8121948242188)), Coord2(552.1951293945313, 664.701171875))
        .curve_to((Coord2(552.0418090820313, 669.23193359375), Coord2(555.0795288085938, 667.7922973632813)), Coord2(555.9325561523438, 664.439697265625))
        .curve_to((Coord2(555.8035278320313, 663.3936767578125), Coord2(543.4547729492188, 664.566162109375)), Coord2(541.8832397460938, 667.4561767578125))
        .curve_to((Coord2(542.26611328125, 672.0006103515625), Coord2(548.4946899414063, 670.5872192382813)), Coord2(547.83984375, 666.5941772460938))
        .curve_to((Coord2(546.2003784179688, 665.4840087890625), Coord2(543.0369262695313, 665.3294677734375)), Coord2(543.1106567382813, 665.9275512695313))
        .curve_to((Coord2(536.3306274414063, 669.7837524414063), Coord2(541.8121337890625, 670.9800415039063)), Coord2(539.4649658203125, 666.6785888671875))
        .curve_to((Coord2(536.6891479492188, 665.93017578125), Coord2(534.8207397460938, 663.9938354492188)), Coord2(533.337890625, 661.9244995117188))
        .curve_to((Coord2(532.1223754882813, 662.1298828125), Coord2(530.9287109375, 662.1915893554688)), Coord2(534.033203125, 663.5484619140625))
        .curve_to((Coord2(539.8789672851563, 669.0048828125), Coord2(535.4338989257813, 664.3715209960938)), Coord2(530.1646118164063, 657.32666015625))
        .curve_to((Coord2(525.6614379882813, 654.2191162109375), Coord2(526.3388671875, 656.8445434570313)), Coord2(530.332275390625, 658.5115356445313))
        .curve_to((Coord2(530.9607543945313, 663.3235473632813), Coord2(535.1883544921875, 667.216552734375)), Coord2(533.1292724609375, 661.65673828125))
        .curve_to((Coord2(526.8078002929688, 654.7847290039063), Coord2(527.2481689453125, 655.82421875)), Coord2(528.5620727539063, 658.5321044921875))
        .curve_to((Coord2(529.048828125, 663.075927734375), Coord2(530.8765869140625, 662.1258544921875)), Coord2(531.5584106445313, 659.6661987304688))
        .curve_to((Coord2(530.1249389648438, 657.940185546875), Coord2(529.1561889648438, 657.2536010742188)), Coord2(528.7389526367188, 655.5059814453125))
        .curve_to((Coord2(527.8021240234375, 654.7122192382813), Coord2(529.899658203125, 656.5814819335938)), Coord2(531.8333740234375, 654.7963256835938))
        .curve_to((Coord2(538.0204467773438, 653.547119140625), Coord2(542.1532592773438, 652.2764892578125)), Coord2(544.957275390625, 652.1034545898438))
        .curve_to((Coord2(545.7479858398438, 652.0574340820313), Coord2(546.3248291015625, 651.8165283203125)), Coord2(546.8508911132813, 651.8157958984375))
        .curve_to((Coord2(548.2747802734375, 652.2127685546875), Coord2(548.1990356445313, 651.2047119140625)), Coord2(547.912109375, 650.8655395507813))
        .curve_to((Coord2(547.7791748046875, 650.193359375), Coord2(549.1414184570313, 650.476806640625)), Coord2(548.0958251953125, 650.5689086914063))
        .curve_to((Coord2(548.0958251953125, 650.7786865234375), Coord2(548.0958251953125, 651.0584716796875)), Coord2(548.0958251953125, 651.2682495117188))
        .curve_to((Coord2(548.9656982421875, 651.643798828125), Coord2(547.8914184570313, 651.3145141601563)), Coord2(549.1207275390625, 650.8655395507813))
        .curve_to((Coord2(548.8338012695313, 650.4108276367188), Coord2(547.700927734375, 649.2344360351563)), Coord2(546.8508911132813, 649.63134765625))
        .curve_to((Coord2(546.3272705078125, 649.630615234375), Coord2(545.5951538085938, 649.4255981445313)), Coord2(544.8019409179688, 649.4730834960938))
        .curve_to((Coord2(542.03857421875, 649.6287841796875), Coord2(537.2066040039063, 649.3989868164063)), Coord2(530.6641845703125, 650.7567138671875))
        .curve_to((Coord2(529.2568359375, 650.2000122070313), Coord2(525.3572998046875, 653.1232299804688)), Coord2(525.4530639648438, 656.3499145507813))
        .curve_to((Coord2(526.0859985351563, 658.6912231445313), Coord2(527.9020385742188, 660.7957763671875)), Coord2(529.1198120117188, 661.9281616210938))
        .curve_to((Coord2(532.0623779296875, 661.90576171875), Coord2(533.4664306640625, 660.0416259765625)), Coord2(529.3554077148438, 657.97998046875))
        .curve_to((Coord2(526.156005859375, 654.2037353515625), Coord2(522.1826782226563, 656.4036254882813)), Coord2(530.3896484375, 664.3438720703125))
        .curve_to((Coord2(536.754150390625, 667.3721923828125), Coord2(535.8456420898438, 660.3375854492188)), Coord2(530.91162109375, 658.0571899414063))
        .curve_to((Coord2(529.3756103515625, 652.6741333007813), Coord2(525.1596069335938, 652.45458984375)), Coord2(527.2052612304688, 659.278076171875))
        .curve_to((Coord2(532.2788696289063, 667.9177856445313), Coord2(540.3832397460938, 669.9564208984375)), Coord2(534.7332763671875, 662.905029296875))
        .curve_to((Coord2(532.432373046875, 658.3805541992188), Coord2(530.3565063476563, 660.47900390625)), Coord2(530.7684326171875, 663.4412841796875))
        .curve_to((Coord2(531.7405395507813, 665.5307006835938), Coord2(535.3882446289063, 669.1942138671875)), Coord2(538.6748046875, 669.9224853515625))
        .curve_to((Coord2(545.757080078125, 667.9179077148438), Coord2(541.6903686523438, 667.3967895507813)), Coord2(543.7351684570313, 669.8466186523438))
        .curve_to((Coord2(545.7384643554688, 670.13720703125), Coord2(545.3059692382813, 669.0546875)), Coord2(544.8681640625, 669.27587890625))
        .curve_to((Coord2(544.5274047851563, 665.6309204101563), Coord2(545.35498046875, 665.0091552734375)), Coord2(545.9674682617188, 668.1023559570313))
        .curve_to((Coord2(544.9426879882813, 667.5361328125), Coord2(553.4862670898438, 669.1529541015625)), Coord2(556.2109375, 667.8751831054688))
        .curve_to((Coord2(557.3668823242188, 664.4981079101563), Coord2(550.9618530273438, 662.6256103515625)), Coord2(549.96337890625, 666.1478271484375))
        .curve_to((Coord2(551.0449829101563, 667.5820922851563), Coord2(551.2767333984375, 667.6608276367188)), Coord2(551.4939575195313, 667.7685546875))
        .curve_to((Coord2(554.0316772460938, 668.719970703125), Coord2(560.2760620117188, 670.0292358398438)), Coord2(561.5628051757813, 670.177978515625))
        .curve_to((Coord2(562.9513549804688, 668.7757568359375), Coord2(560.3701782226563, 666.8861694335938)), Coord2(559.4100952148438, 668.6519775390625))
        .curve_to((Coord2(559.3812255859375, 668.7340698242188), Coord2(559.3759765625, 668.8223876953125)), Coord2(559.3749389648438, 668.913818359375))
        .curve_to((Coord2(559.663330078125, 669.5628662109375), Coord2(561.01806640625, 670.2659912109375)), Coord2(561.4695434570313, 669.9596557617188))
        .curve_to((Coord2(561.845458984375, 670.0281982421875), Coord2(562.2411499023438, 669.9994506835938)), Coord2(562.5125732421875, 670.0426025390625))
        .curve_to((Coord2(562.97314453125, 670.3184814453125), Coord2(562.8713989257813, 669.9105834960938)), Coord2(562.6882934570313, 669.6962890625))
        .curve_to((Coord2(562.89306640625, 669.4701538085938), Coord2(563.1842041015625, 669.1715698242188)), Coord2(562.8291015625, 669.4080810546875))
        .curve_to((Coord2(562.6779174804688, 669.4080810546875), Coord2(562.442626953125, 669.45654296875)), Coord2(562.085693359375, 669.44482421875))
        .build();

    // Create the graph path from the source side
    let mut merged_path = GraphPath::new();
    merged_path         = merged_path.merge(GraphPath::from_merged_paths(vec![&curve].into_iter().map(|path| (path, PathLabel(PathSource::Path1, PathDirection::from(path))))));

    // Collide the path with itself to find the intersections
    merged_path.self_collide(0.01);
    merged_path.set_exterior_by_removing_interior_points();
    merged_path.heal_exterior_gaps();
    println!("{}", graph_path_svg_string(&merged_path, vec![]));

    println!("{:?}", svg_path_string(&curve));
    let with_points_removed: Vec<SimpleBezierPath> = path_remove_interior_points(&vec![curve], 0.01);

    println!("{:?}", with_points_removed.iter()
        .map(|path| svg_path_string(path))
        .collect::<Vec<_>>());

    assert!(with_points_removed.len() > 0);
}


#[test]
fn remove_interior_points_1_without_failing_section() {
    // Complicated curve found in FlowBetween that produces 0 points when interior points are removed, variant with the section that was causing a failure removed
    let curve = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(562.0692138671875, 669.944580078125))
        .curve_to((Coord2(562.0692138671875, 669.944580078125), Coord2(562.0692138671875, 669.944580078125)), Coord2(562.0692138671875, 669.944580078125))
        .curve_to((Coord2(562.4200439453125, 669.9562377929688), Coord2(562.6718139648438, 670.0160522460938)), Coord2(562.8291015625, 670.0160522460938))
        .curve_to((Coord2(562.7747802734375, 670.2525634765625), Coord2(563.1968383789063, 669.9431762695313)), Coord2(563.401611328125, 669.6962890625))
        .curve_to((Coord2(563.218505859375, 669.447021484375), Coord2(562.7468872070313, 668.9757690429688)), Coord2(562.6525268554688, 669.1633911132813))
        .curve_to((Coord2(562.3690185546875, 669.1181640625), Coord2(562.02490234375, 668.9761352539063)), Coord2(561.6610107421875, 668.9097290039063))
        .curve_to((Coord2(560.6327514648438, 668.3336181640625), Coord2(560.5078125, 668.680419921875)), Coord2(560.7962036132813, 668.913818359375))
        .curve_to((Coord2(560.7932739257813, 669.053955078125), Coord2(560.795166015625, 668.9855346679688)), Coord2(560.7711791992188, 669.1161499023438))
        .curve_to((Coord2(560.0169067382813, 670.2783813476563), Coord2(561.4442749023438, 669.9208984375)), Coord2(561.7700805664063, 668.3951416015625))
        .curve_to((Coord2(560.5978393554688, 668.2579956054688), Coord2(555.1843872070313, 665.8880615234375)), Coord2(552.6854248046875, 664.9908447265625))
        .curve_to((Coord2(552.62158203125, 664.9489135742188), Coord2(552.4188842773438, 664.8121948242188)), Coord2(552.1951293945313, 664.701171875))
        .curve_to((Coord2(552.0418090820313, 669.23193359375), Coord2(555.0795288085938, 667.7922973632813)), Coord2(555.9325561523438, 664.439697265625))
        .curve_to((Coord2(555.8035278320313, 663.3936767578125), Coord2(543.4547729492188, 664.566162109375)), Coord2(541.8832397460938, 667.4561767578125))
        .curve_to((Coord2(542.26611328125, 672.0006103515625), Coord2(548.4946899414063, 670.5872192382813)), Coord2(547.83984375, 666.5941772460938))
        .curve_to((Coord2(546.2003784179688, 665.4840087890625), Coord2(543.0369262695313, 665.3294677734375)), Coord2(543.1106567382813, 665.9275512695313))
        .curve_to((Coord2(536.3306274414063, 669.7837524414063), Coord2(541.8121337890625, 670.9800415039063)), Coord2(539.4649658203125, 666.6785888671875))
        .curve_to((Coord2(536.6891479492188, 665.93017578125), Coord2(534.8207397460938, 663.9938354492188)), Coord2(533.337890625, 661.9244995117188))
        .curve_to((Coord2(532.1223754882813, 662.1298828125), Coord2(530.9287109375, 662.1915893554688)), Coord2(534.033203125, 663.5484619140625))
        .curve_to((Coord2(539.8789672851563, 669.0048828125), Coord2(535.4338989257813, 664.3715209960938)), Coord2(530.1646118164063, 657.32666015625))
        .curve_to((Coord2(525.6614379882813, 654.2191162109375), Coord2(526.3388671875, 656.8445434570313)), Coord2(530.332275390625, 658.5115356445313))
        .curve_to((Coord2(530.9607543945313, 663.3235473632813), Coord2(535.1883544921875, 667.216552734375)), Coord2(533.1292724609375, 661.65673828125))
        .curve_to((Coord2(526.8078002929688, 654.7847290039063), Coord2(527.2481689453125, 655.82421875)), Coord2(528.5620727539063, 658.5321044921875))
        .curve_to((Coord2(529.048828125, 663.075927734375), Coord2(530.8765869140625, 662.1258544921875)), Coord2(531.5584106445313, 659.6661987304688))
        .curve_to((Coord2(530.1249389648438, 657.940185546875), Coord2(529.1561889648438, 657.2536010742188)), Coord2(528.7389526367188, 655.5059814453125))
        .curve_to((Coord2(527.8021240234375, 654.7122192382813), Coord2(529.899658203125, 656.5814819335938)), Coord2(531.8333740234375, 654.7963256835938))
        .curve_to((Coord2(538.0204467773438, 653.547119140625), Coord2(542.1532592773438, 652.2764892578125)), Coord2(544.957275390625, 652.1034545898438))
        .curve_to((Coord2(545.7479858398438, 652.0574340820313), Coord2(546.3248291015625, 651.8165283203125)), Coord2(546.8508911132813, 651.8157958984375))
        .curve_to((Coord2(548.2747802734375, 652.2127685546875), Coord2(548.1990356445313, 651.2047119140625)), Coord2(547.912109375, 650.8655395507813))
        .curve_to((Coord2(547.7791748046875, 650.193359375), Coord2(549.1414184570313, 650.476806640625)), Coord2(548.0958251953125, 650.5689086914063))
        .curve_to((Coord2(548.0958251953125, 650.7786865234375), Coord2(548.0958251953125, 651.0584716796875)), Coord2(548.0958251953125, 651.2682495117188))
        .curve_to((Coord2(548.9656982421875, 651.643798828125), Coord2(547.8914184570313, 651.3145141601563)), Coord2(549.1207275390625, 650.8655395507813))
        .curve_to((Coord2(548.8338012695313, 650.4108276367188), Coord2(547.700927734375, 649.2344360351563)), Coord2(546.8508911132813, 649.63134765625))
        .curve_to((Coord2(546.3272705078125, 649.630615234375), Coord2(545.5951538085938, 649.4255981445313)), Coord2(544.8019409179688, 649.4730834960938))
        .curve_to((Coord2(542.03857421875, 649.6287841796875), Coord2(537.2066040039063, 649.3989868164063)), Coord2(530.6641845703125, 650.7567138671875))
        .curve_to((Coord2(529.2568359375, 650.2000122070313), Coord2(525.3572998046875, 653.1232299804688)), Coord2(525.4530639648438, 656.3499145507813))
        .curve_to((Coord2(526.0859985351563, 658.6912231445313), Coord2(527.9020385742188, 660.7957763671875)), Coord2(529.1198120117188, 661.9281616210938))
        .curve_to((Coord2(532.0623779296875, 661.90576171875), Coord2(533.4664306640625, 660.0416259765625)), Coord2(529.3554077148438, 657.97998046875))
        .curve_to((Coord2(526.156005859375, 654.2037353515625), Coord2(522.1826782226563, 656.4036254882813)), Coord2(530.3896484375, 664.3438720703125))
        .curve_to((Coord2(536.754150390625, 667.3721923828125), Coord2(535.8456420898438, 660.3375854492188)), Coord2(530.91162109375, 658.0571899414063))
        .curve_to((Coord2(529.3756103515625, 652.6741333007813), Coord2(525.1596069335938, 652.45458984375)), Coord2(527.2052612304688, 659.278076171875))
        .curve_to((Coord2(532.2788696289063, 667.9177856445313), Coord2(540.3832397460938, 669.9564208984375)), Coord2(534.7332763671875, 662.905029296875))
        .curve_to((Coord2(532.432373046875, 658.3805541992188), Coord2(530.3565063476563, 660.47900390625)), Coord2(530.7684326171875, 663.4412841796875))
        /*FAIL*//*.curve_to((Coord2(531.7405395507813, 665.5307006835938), Coord2(535.3882446289063, 669.1942138671875)), Coord2(538.6748046875, 669.9224853515625))*/
        .curve_to((Coord2(545.757080078125, 667.9179077148438), Coord2(541.6903686523438, 667.3967895507813)), Coord2(543.7351684570313, 669.8466186523438))
        .curve_to((Coord2(545.7384643554688, 670.13720703125), Coord2(545.3059692382813, 669.0546875)), Coord2(544.8681640625, 669.27587890625))
        .curve_to((Coord2(544.5274047851563, 665.6309204101563), Coord2(545.35498046875, 665.0091552734375)), Coord2(545.9674682617188, 668.1023559570313))
        .curve_to((Coord2(544.9426879882813, 667.5361328125), Coord2(553.4862670898438, 669.1529541015625)), Coord2(556.2109375, 667.8751831054688))
        .curve_to((Coord2(557.3668823242188, 664.4981079101563), Coord2(550.9618530273438, 662.6256103515625)), Coord2(549.96337890625, 666.1478271484375))
        .curve_to((Coord2(551.0449829101563, 667.5820922851563), Coord2(551.2767333984375, 667.6608276367188)), Coord2(551.4939575195313, 667.7685546875))
        .curve_to((Coord2(554.0316772460938, 668.719970703125), Coord2(560.2760620117188, 670.0292358398438)), Coord2(561.5628051757813, 670.177978515625))
        .curve_to((Coord2(562.9513549804688, 668.7757568359375), Coord2(560.3701782226563, 666.8861694335938)), Coord2(559.4100952148438, 668.6519775390625))
        .curve_to((Coord2(559.3812255859375, 668.7340698242188), Coord2(559.3759765625, 668.8223876953125)), Coord2(559.3749389648438, 668.913818359375))
        .curve_to((Coord2(559.663330078125, 669.5628662109375), Coord2(561.01806640625, 670.2659912109375)), Coord2(561.4695434570313, 669.9596557617188))
        .curve_to((Coord2(561.845458984375, 670.0281982421875), Coord2(562.2411499023438, 669.9994506835938)), Coord2(562.5125732421875, 670.0426025390625))
        .curve_to((Coord2(562.97314453125, 670.3184814453125), Coord2(562.8713989257813, 669.9105834960938)), Coord2(562.6882934570313, 669.6962890625))
        .curve_to((Coord2(562.89306640625, 669.4701538085938), Coord2(563.1842041015625, 669.1715698242188)), Coord2(562.8291015625, 669.4080810546875))
        .curve_to((Coord2(562.6779174804688, 669.4080810546875), Coord2(562.442626953125, 669.45654296875)), Coord2(562.085693359375, 669.44482421875))
        .build();

    println!("{:?}", svg_path_string(&curve));
    let with_points_removed: Vec<SimpleBezierPath> = path_remove_interior_points(&vec![curve], 0.01);

    println!("{:?}", with_points_removed.iter()
        .map(|path| svg_path_string(path))
        .collect::<Vec<_>>());

    assert!(with_points_removed.len() > 0);
}

