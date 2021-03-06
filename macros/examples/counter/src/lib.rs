use pd_ext::builder::ControlExternalBuilder;
use pd_ext::external::ControlExternal;
use pd_ext::outlet::{OutletSend, OutletType};
use pd_ext_macros::external;

//based on https://github.com/pure-data/externals-howto#a-simple-external-counter
external! {
    #[repr(packed)]
    pub struct Counter {
        count: isize,
        outlet: Box<dyn OutletSend>
    }

    impl ControlExternal for Counter {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Result<Self, String> {
            let count = if let Some(atom) = builder.creation_args().iter().next() {
                atom.get_int().unwrap_or(0) as isize
            } else {
                0isize
            };
            let outlet = builder.new_message_outlet(OutletType::Float);
            Ok(Self { count, outlet })
        }
    }

    impl Counter {
        #[bang]
        pub fn bang(&mut self) {
            let f = self.count as pd_sys::t_float;
            self.count = self.count.wrapping_add(1);
            self.outlet.send_float(f);
        }
    }
}
