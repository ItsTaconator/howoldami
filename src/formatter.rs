use humanize_duration::types::DurationParts;
use humanize_duration::*;

pub(crate) struct CustomFormatter;
unit!(MyYear, " year,", " years,");
unit!(MyMonth, " month,", " months,");
unit!(MyDay, " day,", " days,");
unit!(MyHour, " hour,", " hours,");
unit!(MyMinute, " minute,", " minutes,");
unit!(MySecond, " second,", " seconds,");
unit!(MyMilli, " millisecond,", " milliseconds,");
unit!(MyMicro, " microsecond,", " microseconds,");
unit!(MyNano, " nanosecond,", " nanoseconds,");

impl Formatter for CustomFormatter {
    fn get(&self, truncate: Truncate) -> Box<dyn Unit> {
        match truncate {
            Truncate::Nano => Box::new(MyNano),
            Truncate::Micro => Box::new(MyMicro),
            Truncate::Millis => Box::new(MyMilli),
            Truncate::Second => Box::new(MySecond),
            Truncate::Minute => Box::new(MyMinute),
            Truncate::Hour => Box::new(MyHour),
            Truncate::Day => Box::new(MyDay),
            Truncate::Month => Box::new(MyMonth),
            Truncate::Year => Box::new(MyYear),
        }
    }

    fn format(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        parts: DurationParts,
        truncate: Truncate,
    ) -> std::fmt::Result {
        self.format_default(f, parts, truncate)
    }
}
