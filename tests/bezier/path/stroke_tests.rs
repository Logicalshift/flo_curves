use flo_curves::geo::*;
use flo_curves::bezier::*;
use flo_curves::bezier::path::*;

#[test]
fn stroke_closes_path_1() {
    let source_path     = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(-0.4241647720336914, -0.5655260682106018))
        .curve_to((Coord2(-0.42449551820755005, -0.5624192953109741), Coord2(-0.42483407258987427, -0.5598203539848328)), Coord2(-0.42421942949295044, -0.5567578077316284))
        .curve_to((Coord2(-0.4213106632232666, -0.5422917008399963), Coord2(-0.4013028144836426, -0.541242241859436)), Coord2(-0.3897610902786255, -0.5445390939712524))
        .curve_to((Coord2(-0.38367515802383423, -0.5462786555290222), Coord2(-0.37594079971313477, -0.5489739775657654)), Coord2(-0.37083661556243896, -0.5525599122047424))
        .curve_to((Coord2(-0.3617064952850342, -0.5589792132377625), Coord2(-0.3560839891433716, -0.566072940826416)), Coord2(-0.3501152992248535, -0.5752266049385071))
        .curve_to((Coord2(-0.3474694490432739, -0.5792812705039978), Coord2(-0.3451542854309082, -0.5823802351951599)), Coord2(-0.3430267572402954, -0.5867734551429749))
        .curve_to((Coord2(-0.3401283025741577, -0.5927551984786987), Coord2(-0.3379720449447632, -0.5992813110351563)), Coord2(-0.3365788459777832, -0.6057526469230652))
        .curve_to((Coord2(-0.33605802059173584, -0.6081719398498535), Coord2(-0.33473503589630127, -0.6122525930404663)), Coord2(-0.33617258071899414, -0.6146484613418579))
        .curve_to((Coord2(-0.33695387840270996, -0.6159505844116211), Coord2(-0.3390554189682007, -0.6160208582878113)), Coord2(-0.340242862701416, -0.61677086353302))
        .curve_to((Coord2(-0.34509706497192383, -0.6198411583900452), Coord2(-0.36623769998550415, -0.6199765801429749)), Coord2(-0.37046170234680176, -0.6194036602973938))
        .curve_to((Coord2(-0.38461530208587646, -0.6174792051315308), Coord2(-0.39963871240615845, -0.611023485660553)), Coord2(-0.40813350677490234, -0.599109411239624))
        .curve_to((Coord2(-0.413180410861969, -0.5920312404632568), Coord2(-0.41695642471313477, -0.5837708711624146)), Coord2(-0.42010748386383057, -0.5756927728652954))
        .curve_to((Coord2(-0.4211726188659668, -0.5729583501815796), Coord2(-0.422487735748291, -0.5704115033149719)), Coord2(-0.4234902858734131, -0.5676692724227905))
        .curve_to((Coord2(-0.42384183406829834, -0.5667083263397217), Coord2(-0.4238600730895996, -0.5657370090484619)), Coord2(-0.4242611527442932, -0.56480473279953))
        .curve_to((Coord2(-0.4243835210800171, -0.5645208358764648), Coord2(-0.4241647720336914, -0.5653906464576721)), Coord2(-0.4241647720336914, -0.5655260682106018))
        .build();
    let width       = 0.0026041667442768812;
    let options     = StrokeOptions::default()
        .with_accuracy(0.002)
        .with_min_sample_distance(0.001)
        .with_join(LineJoin::Bevel)
        .with_start_cap(LineCap::Butt)
        .with_end_cap(LineCap::Butt);

    let stroked_path = stroke_path::<SimpleBezierPath, _>(&source_path, width, &options);

    assert!(stroked_path.len() == 1, "Should be 1 subpath, found {}", stroked_path.len());
    
    let stroked_path    = &stroked_path[0];
    let curves          = stroked_path.to_curves::<Curve<_>>();

    assert!(curves.len() > 0, "Subpath should have at least one curve");

    let start_point = curves[0].start_point();
    let end_point   = curves.last().unwrap().end_point();
    assert!(end_point == start_point, "Path should be closed ({:?} != {:?}), curves are {:?}", start_point, end_point, curves);
}
