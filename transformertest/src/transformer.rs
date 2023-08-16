//! Transformers framework.

use tokio::task::JoinHandle;

#[async_trait::async_trait]
pub trait Generator {
    type Output;

    async fn next(&self) -> Option<Self::Output>;
}

pub trait Processor {
    type Input;
    type Output;

    fn transform(&self, data: Self::Input) -> Self::Output;
}

pub struct Transformer<P: Processor> {
    chain: Vec<Transformer<dyn Processor<Input = P::Output>>>,
    processor: P,
}

pub struct Source<G> where G: Generator {
    chain: Vec<Transformer<dyn Processor<Input = G::Output>>>,
    generator: G,
}

impl<G: Generator> Source<G> {
    pub fn new(gen: G) -> Self {
        Self {
            chain: vec![],
            generator: gen,
        }
    }

    pub fn chain(&mut self, next: Transformer<G::Output> ) {
        self.chain.push(next);
    }

    // pub async fn spawn(&self) -> JoinHandle<()> {
    //     tokio::spawn(async {
    //         while let Some(data) = gen.next().await {
    //             for next in self.chain {
    //                 next.send(data).await.unwrap();
    //             }
    //         }
    //     })
    // }
}


// impl<P: Processor> Transformer<P> {
//     pub fn new(proc: P) -> Self {
//         Self {
//             chain: vec![],
//             processor: proc
//         }
//     }

//     pub fn chain(&mut self, next: Transformer<P::Output>) {
//         self.chain.push(next);
//     }
// }
