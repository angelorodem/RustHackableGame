pub mod GameNetworking {

    pub use crate::structures::Structures::*;
    pub use crate::AskForPlayer_generated::{AskForPlayer,AskForPlayerArgs, get_root_as_ask_for_player};

    pub fn hello(){
        println!("Module Works");
    }

    pub fn ask_for_player(name: &String, password: &String){
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);
        let fname = builder.create_string(&name);
        let fpassword = builder.create_string(&password);

        let afp_packet = AskForPlayer::create(&mut builder, &AskForPlayerArgs{
            name: Some(fname),
            password: Some(fpassword)
        });

        builder.finish(afp_packet, None);

        let buf = builder.finished_data();

        println!("{:?}",&buf);

        let afp_packet_re = get_root_as_ask_for_player(buf);

        println!("N:{:?} P:{:?}",afp_packet_re.name(), afp_packet_re.password());

    }

    pub fn send_message(player: &Player, message: String){

    }

    pub fn send_score(player: &Player){

    }
}