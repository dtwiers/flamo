use core::panic;

use num::Float;
use rand::Rng;

use super::{variations::*, Affine, Variation};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ComputeParameters<Scalar: Float> {
    pub total_weight: Option<Scalar>,
    pub final_variation: Option<FinalVariation<Scalar>>,
    pub post_transform: Affine<Scalar>,
    pub linear: LinearVariation<Scalar>,
    pub sinusoidal: SinusoidalVariation<Scalar>,
    pub spherical: SphericalVariation<Scalar>,
    pub swirl: SwirlVariation<Scalar>,
    pub horseshoe: HorseshoeVariation<Scalar>,
    pub polar: PolarVariation<Scalar>,
    pub handkerchief: HandkerchiefVariation<Scalar>,
    pub heart: HeartVariation<Scalar>,
    pub disc: DiscVariation<Scalar>,
    pub spiral: SpiralVariation<Scalar>,
    pub hyperbolic: HyperbolicVariation<Scalar>,
    pub diamond: DiamondVariation<Scalar>,
    pub ex: ExVariation<Scalar>,
    pub julia: JuliaVariation<Scalar>,
    pub bent: BentVariation<Scalar>,
    pub waves: WavesVariation<Scalar>,
    pub fisheye: FisheyeVariation<Scalar>,
    pub popcorn: PopcornVariation<Scalar>,
    pub exponential: ExponentialVariation<Scalar>,
    pub power: PowerVariation<Scalar>,
    pub cosine: CosineVariation<Scalar>,
    pub rings: RingsVariation<Scalar>,
    pub fan: FanVariation<Scalar>,
    pub blob: BlobVariation<Scalar>,
    pub pdj: PDJVariation<Scalar>,
    pub fan2: Fan2Variation<Scalar>,
    pub rings2: Rings2Variation<Scalar>,
    pub eyefish: EyefishVariation<Scalar>,
    pub bubble: BubbleVariation<Scalar>,
    pub cylinder: CylinderVariation<Scalar>,
    pub perspective: PerspectiveVariation<Scalar>,
    pub noise: NoiseVariation<Scalar>,
    pub julia_n: JuliaNVariation<Scalar>,
    pub julia_scope: JuliaScopeVariation<Scalar>,
    pub blur: BlurVariation<Scalar>,
    pub gaussian: GaussianVariation<Scalar>,
    pub radial_blur: RadialBlurVariation<Scalar>,
    pub pie: PieVariation<Scalar>,
    pub ngon: NgonVariation<Scalar>,
    pub curl: CurlVariation<Scalar>,
    pub rectangles: RectanglesVariation<Scalar>,
    pub arch: ArchVariation<Scalar>,
    pub tangent: TangentVariation<Scalar>,
    pub square: SquareVariation<Scalar>,
    pub rays: RaysVariation<Scalar>,
    pub blade: BladeVariation<Scalar>,
    pub secant: SecantVariation<Scalar>,
    pub twintrian: TwintrianVariation<Scalar>,
    pub cross: CrossVariation<Scalar>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FinalVariation<Scalar: Float> {
    Linear(LinearVariation<Scalar>),
    Spherical(SphericalVariation<Scalar>),
    Swirl(SwirlVariation<Scalar>),
    Horseshoe(HorseshoeVariation<Scalar>),
    Polar(PolarVariation<Scalar>),
    Handkerchief(HandkerchiefVariation<Scalar>),
    Heart(HeartVariation<Scalar>),
    Disc(DiscVariation<Scalar>),
    Spiral(SpiralVariation<Scalar>),
    Hyperbolic(HyperbolicVariation<Scalar>),
    Diamond(DiamondVariation<Scalar>),
    Ex(ExVariation<Scalar>),
    Julia(JuliaVariation<Scalar>),
    Bent(BentVariation<Scalar>),
    Waves(WavesVariation<Scalar>),
    Fisheye(FisheyeVariation<Scalar>),
    Popcorn(PopcornVariation<Scalar>),
    Exponential(ExponentialVariation<Scalar>),
    Power(PowerVariation<Scalar>),
    Cosine(CosineVariation<Scalar>),
    Rings(RingsVariation<Scalar>),
    Fan(FanVariation<Scalar>),
    Blob(BlobVariation<Scalar>),
    Pdj(PDJVariation<Scalar>),
    Fan2(Fan2Variation<Scalar>),
    Rings2(Rings2Variation<Scalar>),
    Eyefish(EyefishVariation<Scalar>),
    Bubble(BubbleVariation<Scalar>),
    Cylinder(CylinderVariation<Scalar>),
    Perspective(PerspectiveVariation<Scalar>),
    Noise(NoiseVariation<Scalar>),
    JuliaN(JuliaNVariation<Scalar>),
    JuliaScope(JuliaScopeVariation<Scalar>),
    Blur(BlurVariation<Scalar>),
    Gaussian(GaussianVariation<Scalar>),
    RadialBlur(RadialBlurVariation<Scalar>),
    Pie(PieVariation<Scalar>),
    Ngon(NgonVariation<Scalar>),
    Curl(CurlVariation<Scalar>),
    Rectangles(RectanglesVariation<Scalar>),
    Arch(ArchVariation<Scalar>),
    Tangent(TangentVariation<Scalar>),
    Square(SquareVariation<Scalar>),
    Rays(RaysVariation<Scalar>),
    Blade(BladeVariation<Scalar>),
    Secant(SecantVariation<Scalar>),
    Twintrian(TwintrianVariation<Scalar>),
    Cross(CrossVariation<Scalar>),
}

impl<Scalar: Float> ComputeParameters<Scalar> {
    pub fn init_weight(&self) -> Self {
        let mut total_weight = Scalar::zero();
        total_weight = total_weight + self.linear.weight;
        total_weight = total_weight + self.sinusoidal.weight;
        total_weight = total_weight + self.spherical.weight;
        total_weight = total_weight + self.swirl.weight;
        total_weight = total_weight + self.horseshoe.weight;
        total_weight = total_weight + self.polar.weight;
        total_weight = total_weight + self.handkerchief.weight;
        total_weight = total_weight + self.heart.weight;
        total_weight = total_weight + self.disc.weight;
        total_weight = total_weight + self.spiral.weight;
        total_weight = total_weight + self.hyperbolic.weight;
        total_weight = total_weight + self.diamond.weight;
        total_weight = total_weight + self.ex.weight;
        total_weight = total_weight + self.julia.weight;
        total_weight = total_weight + self.bent.weight;
        total_weight = total_weight + self.waves.weight;
        total_weight = total_weight + self.fisheye.weight;
        total_weight = total_weight + self.popcorn.weight;
        total_weight = total_weight + self.exponential.weight;
        total_weight = total_weight + self.power.weight;
        total_weight = total_weight + self.cosine.weight;
        total_weight = total_weight + self.rings.weight;
        total_weight = total_weight + self.fan.weight;
        total_weight = total_weight + self.blob.weight;
        total_weight = total_weight + self.pdj.weight;
        total_weight = total_weight + self.fan2.weight;
        total_weight = total_weight + self.rings2.weight;
        total_weight = total_weight + self.eyefish.weight;
        total_weight = total_weight + self.bubble.weight;
        total_weight = total_weight + self.cylinder.weight;
        total_weight = total_weight + self.perspective.weight;
        total_weight = total_weight + self.noise.weight;
        total_weight = total_weight + self.julia_n.weight;
        total_weight = total_weight + self.julia_scope.weight;
        total_weight = total_weight + self.blur.weight;
        total_weight = total_weight + self.gaussian.weight;
        total_weight = total_weight + self.radial_blur.weight;
        total_weight = total_weight + self.pie.weight;
        total_weight = total_weight + self.ngon.weight;
        total_weight = total_weight + self.curl.weight;
        total_weight = total_weight + self.rectangles.weight;
        total_weight = total_weight + self.arch.weight;
        total_weight = total_weight + self.tangent.weight;
        total_weight = total_weight + self.square.weight;
        total_weight = total_weight + self.rays.weight;
        total_weight = total_weight + self.blade.weight;
        total_weight = total_weight + self.secant.weight;
        total_weight = total_weight + self.twintrian.weight;
        total_weight = total_weight + self.cross.weight;

        let mut result = self.clone();
        result.total_weight = Some(total_weight);
        result
    }

    pub fn choose(&self) -> Box<&dyn Variation<Scalar>> {
        let mut rng = rand::thread_rng();
        let variation = [
            &self.linear as &dyn Variation<Scalar>,
            &self.sinusoidal as &dyn Variation<Scalar>,
            &self.spherical as &dyn Variation<Scalar>,
            &self.swirl as &dyn Variation<Scalar>,
            &self.horseshoe as &dyn Variation<Scalar>,
            &self.polar as &dyn Variation<Scalar>,
            &self.handkerchief as &dyn Variation<Scalar>,
            &self.heart as &dyn Variation<Scalar>,
            &self.disc as &dyn Variation<Scalar>,
            &self.spiral as &dyn Variation<Scalar>,
            &self.hyperbolic as &dyn Variation<Scalar>,
            &self.diamond as &dyn Variation<Scalar>,
            &self.ex as &dyn Variation<Scalar>,
            &self.julia as &dyn Variation<Scalar>,
            &self.bent as &dyn Variation<Scalar>,
            &self.waves as &dyn Variation<Scalar>,
            &self.fisheye as &dyn Variation<Scalar>,
            &self.popcorn as &dyn Variation<Scalar>,
            &self.exponential as &dyn Variation<Scalar>,
            &self.power as &dyn Variation<Scalar>,
            &self.cosine as &dyn Variation<Scalar>,
            &self.rings as &dyn Variation<Scalar>,
            &self.fan as &dyn Variation<Scalar>,
            &self.blob as &dyn Variation<Scalar>,
            &self.pdj as &dyn Variation<Scalar>,
            &self.fan2 as &dyn Variation<Scalar>,
            &self.rings2 as &dyn Variation<Scalar>,
            &self.eyefish as &dyn Variation<Scalar>,
            &self.bubble as &dyn Variation<Scalar>,
            &self.cylinder as &dyn Variation<Scalar>,
            &self.perspective as &dyn Variation<Scalar>,
            &self.noise as &dyn Variation<Scalar>,
            &self.julia_n as &dyn Variation<Scalar>,
            &self.julia_scope as &dyn Variation<Scalar>,
            &self.blur as &dyn Variation<Scalar>,
            &self.gaussian as &dyn Variation<Scalar>,
            &self.radial_blur as &dyn Variation<Scalar>,
            &self.pie as &dyn Variation<Scalar>,
            &self.ngon as &dyn Variation<Scalar>,
            &self.curl as &dyn Variation<Scalar>,
            &self.rectangles as &dyn Variation<Scalar>,
            &self.arch as &dyn Variation<Scalar>,
            &self.tangent as &dyn Variation<Scalar>,
            &self.square as &dyn Variation<Scalar>,
            &self.rays as &dyn Variation<Scalar>,
            &self.blade as &dyn Variation<Scalar>,
            &self.secant as &dyn Variation<Scalar>,
            &self.twintrian as &dyn Variation<Scalar>,
            &self.cross as &dyn Variation<Scalar>,
        ];

        if self.total_weight.is_none() {
            panic!("total_weight is None");
        }

        let total_weight = self.total_weight.unwrap();

        let chosen_weight = rng.gen_range(0.0..(total_weight.to_f64().unwrap()));
        let mut current_weight = Scalar::from(chosen_weight).unwrap();

        for variation in variation.iter() {
            current_weight = current_weight - variation.weight();
            if current_weight <= Scalar::zero() {
                return Box::new(*variation);
            }
        }
        return Box::new(variation.last().map(|x| *x).unwrap());
    }
}
