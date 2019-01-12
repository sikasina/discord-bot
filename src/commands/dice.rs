use rand::Rng;

command!(d(_ctx, msg, args) {
    let die = args.single::<i64>().unwrap();
    let number = args.single::<i64>().unwrap();
    let product = die * number;
    let roll = rand::thread_rng().gen_range(1, product);
    let _ = msg.channel_id.say(roll);
});