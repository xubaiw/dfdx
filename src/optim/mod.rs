//! Optimizers such as [Sgd], [Adam], and [RMSprop] that can optimize neural networks.
//!
//! # Initializing
//!
//! All the optimizer's provide [Default] implementations, and also provide a way to specify
//! all the relevant parameters:
//! - [Sgd::new()]
//! - [Adam::new()]
//! - [RMSprop::new()]
//!
//! # Updating network parameters
//!
//! This is done via [Optimizer::update()], where you pass in a mutable [crate::nn::Module], and
//! the [crate::gradients::Gradients]:
//!
//! ```rust
//! # use dfdx::prelude::*;
//! # type MyModel = Linear<5, 2>;
//! let mut model: MyModel = Default::default();
//! let mut opt: Sgd = Default::default();
//! # let y = model.forward(Tensor1D::zeros().traced());
//! # let loss = mse_loss(y, &Tensor1D::zeros());
//! // -- snip loss computation --
//!
//! let gradients: Gradients = loss.backward();
//! opt.update(&mut model, gradients);
//! ```

mod adam;
mod optimizer;
mod rmsprop;
mod sgd;

pub use adam::*;
pub use optimizer::*;
pub use rmsprop::*;
pub use sgd::*;
