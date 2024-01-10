// Copyright 2021 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Lightweight runtime-agnostic async task scheduler with cron expression support
//!
//! - **Lightweight**: Minimal dependencies because it does not rely on a runtime.
//! - **Efficient**: Tickless design with no reference counters.
//! - **Runtime-Agnostic**: Bring your own runtime. No runtime dependencies.
//! - **Async**: A single future drives the entire scheduler service.
//! - **Task Scheduling**: Schedule multiple jobs with varying timeframes between them.
//! - **Cron Expressions**: Standardized format for scheduling syntax.
//!
//! # Demo
//!
//! ```
//! use chrono::offset::Local;
//! use async_cron_scheduler::*;
//! use smol::Timer;
//! use std::time::Duration;
//!
//! smol::block_on(async move {
//!     // Creates a scheduler based on the Local timezone. Note that the `sched_service`
//!     // contains the background job as a future for the caller to decide how to await
//!     // it. When the scheduler is dropped, the scheduler service will exit as well.
//!     let (mut scheduler, sched_service) = Scheduler::<Local>::launch(Timer::after);
//!
//!     // Creates a job which executes every 1 seconds.
//!     let job = Job::cron("1/1 * * * * *").unwrap();
//!     let fizz_id = scheduler.insert(job, |id| println!("Fizz")).await;
//!
//!     // Creates a job which executes every 3 seconds.
//!     let job = Job::cron("1/3 * * * * *").unwrap();
//!     let buzz_id = scheduler.insert(job, |id| println!("Buzz")).await;
//!
//!     // Creates a job which executes every 5 seconds.
//!     let job = Job::cron("1/5 * * * * *").unwrap();
//!     let bazz_id = scheduler.insert(job, |id| println!("Bazz")).await;
//!
//!     // A future which gradually drops jobs from the scheduler.
//!     let dropper = async move {
//!         Timer::after(Duration::from_secs(7)).await;
//!         scheduler.remove(fizz_id).await;
//!         println!("Fizz gone");
//!         Timer::after(Duration::from_secs(5)).await;
//!         scheduler.remove(buzz_id).await;
//!         println!("Buzz gone");
//!         Timer::after(Duration::from_secs(1)).await;
//!         scheduler.remove(bazz_id).await;
//!         println!("Bazz gone");
//!
//!         // `scheduler` is dropped here, which causes the sched_service to end.
//!     };
//!
//!     // Poll the dropper and scheduler service concurrently until both return.
//!     futures::future::join(sched_service, dropper).await;
//! });
//! ```

use chrono::DateTime;
use chrono::TimeZone;
pub use cron;

mod job;
mod scheduler;

pub use self::job::*;
pub use self::scheduler::*;

/// Extensions for the chrono timezone structs.
pub trait TimeZoneExt: TimeZone + Copy + Clone {
    /// Constructs a default timezone struct for this timezone.
    fn timescale() -> Self;

    /// Get the current time in this timezone.
    fn now() -> DateTime<Self>;
}

impl TimeZoneExt for chrono::Local {
    fn timescale() -> Self {
        Self
    }
    fn now() -> DateTime<Self> {
        Self::now()
    }
}

impl TimeZoneExt for chrono::Utc {
    fn timescale() -> Self {
        Self
    }

    fn now() -> DateTime<Self> {
        Self::now()
    }
}
