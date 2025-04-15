use lrtc::{classify, CompressionAlgorithm};

pub fn check(to_check: &str) -> bool {
    let training = vec![
        String::from("Currently trying not to shit my pants off a farmer's wrap and iced capp"),
        String::from("Experiencing cuteness agression for my cat"),
        String::from("I can't put down the cup, can't put down the cup"),
        String::from("The Carnification of society"),
        String::from("I am unable to do any work without a white monster"),
        String::from("My favorite drink is bubly. Specifically, lime bubly."),
        String::from("Emotional affair, overly sincere. #phoebe bridgers"),
        String::from("Literally what is the problem with these alt-right opps on my timeline"),
        String::from("it is a beautiful day today"),
        String::from("the meta is fake it til you make it"),
        String::from("Adam's leather bag is soft and leathery"),
        String::from("Took a nap today and woke up feeling freaky"),
        String::from("Literally can’t go a day without my iced matcha, it's a problem"),
        String::from("Why does my dog look like he's plotting something sinister 🐶"),
        String::from("I’m about to hit a mid-day slump but all I want is a nap and some fries 🍟"),
        String::from("Everything feels like a soft serve cone in a hurricane"),
        String::from("My brain is 50% caffeine and 50% “what’s for dinner”"),
        String::from(
            "I just tried a new flavor of kombucha and now I'm questioning all my life choices",
        ),
        String::from("Just saw a dog in a sweater and now my whole day is made 🐕🧣"),
        String::from(
            "Mood: I'm about 30 minutes away from taking a nap but also should be productive",
        ),
        String::from(
            "Can't focus today, too busy wondering if I should switch to a new skincare routine",
        ),
        String::from("I just blinked and now I'm 5 years older and somehow a coffee addict ☕"),
    ];
    let training_labels = vec![
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("human"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
        String::from("AI"),
    ];
    let queries = vec![String::from(to_check)];

    let result = classify(
        &training,
        &training_labels,
        &queries,
        3i32,
        CompressionAlgorithm::Gzip,
        1usize,
    );

    if result[0] != "human" {
        return false;
    };

    println!("{}", result[0]);

    true
}
