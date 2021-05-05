//! Job system
use std::{
    cell::RefCell,
    collections::hash_map::{Entry, HashMap},
    sync::mpsc
};
use workerpool::Pool;
use workerpool::Builder;
use workerpool::thunk::{ThunkWorker, Thunk};

struct Job {
    rx: mpsc::Receiver<Output>
}

type Output = String;
type JobId = String;

const NO_RESULTS_YET: &str = "NO RESULTS YET";
const NO_SUCH_JOB: &str = "NO SUCH JOB";
const JOB_PANICKED: &str = "JOB PANICKED";

#[derive(Default)]
struct Jobs {
    map: HashMap<JobId, Job>,
    next_job: usize,
    pool: Pool::<ThunkWorker<Output>>
}

impl Jobs {
    fn start<F: FnOnce() -> Output + Send + 'static>(&mut self, f: F) -> JobId {
        let (tx, rx) = mpsc::channel();
        self.pool.execute_to(tx, Thunk::of(|| f()));
        let id = self.next_job.to_string();
        self.next_job += 1;
        self.map.insert(id.clone(), Job { rx });
        id
    }

    fn check(&mut self, id: &str) -> Output {
        let entry = match self.map.entry(id.to_owned()) {
            Entry::Occupied(occupied) => occupied,
            Entry::Vacant(_) => return NO_SUCH_JOB.to_owned(),
        };
        let result = match entry.get().rx.try_recv() {
            Ok(result) => result,
            Err(mpsc::TryRecvError::Disconnected) => JOB_PANICKED.to_owned(),
            Err(mpsc::TryRecvError::Empty) => return NO_RESULTS_YET.to_owned(),
        };
        let _ = entry.remove();
        result
    }
}

thread_local! {
    static JOBS: RefCell<Jobs> = RefCell::new(
        Jobs {
            map: Default::default(),
            next_job: 0,
            pool: Builder::new().num_threads(64).thread_stack_size(512 * 1024).build()
        }
    )
}

pub fn start<F: FnOnce() -> Output + Send + 'static>(f: F) -> JobId {
    JOBS.with(|jobs| jobs.borrow_mut().start(f))
}

pub fn check(id: &str) -> String {
    JOBS.with(|jobs| jobs.borrow_mut().check(id))
}
