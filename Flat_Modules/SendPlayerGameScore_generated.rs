// automatically generated by the FlatBuffers compiler, do not modify



use crate::GameResult_generated::*;
use crate::Player_generated::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

pub enum SendGameScoreOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct SendGameScore<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SendGameScore<'a> {
    type Inner = SendGameScore<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> SendGameScore<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        SendGameScore {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args SendGameScoreArgs<'args>) -> flatbuffers::WIPOffset<SendGameScore<'bldr>> {
      let mut builder = SendGameScoreBuilder::new(_fbb);
      if let Some(x) = args.score_message { builder.add_score_message(x); }
      if let Some(x) = args.game_result { builder.add_game_result(x); }
      if let Some(x) = args.player { builder.add_player(x); }
      builder.finish()
    }

    pub const VT_PLAYER: flatbuffers::VOffsetT = 4;
    pub const VT_GAME_RESULT: flatbuffers::VOffsetT = 6;
    pub const VT_SCORE_MESSAGE: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn player(&self) -> Option<Player<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Player<'a>>>(SendGameScore::VT_PLAYER, None)
  }
  #[inline]
  pub fn game_result(&self) -> Option<GameResult<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<GameResult<'a>>>(SendGameScore::VT_GAME_RESULT, None)
  }
  #[inline]
  pub fn score_message(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SendGameScore::VT_SCORE_MESSAGE, None)
  }
}

pub struct SendGameScoreArgs<'a> {
    pub player: Option<flatbuffers::WIPOffset<Player<'a >>>,
    pub game_result: Option<flatbuffers::WIPOffset<GameResult<'a >>>,
    pub score_message: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for SendGameScoreArgs<'a> {
    #[inline]
    fn default() -> Self {
        SendGameScoreArgs {
            player: None,
            game_result: None,
            score_message: None,
        }
    }
}
pub struct SendGameScoreBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SendGameScoreBuilder<'a, 'b> {
  #[inline]
  pub fn add_player(&mut self, player: flatbuffers::WIPOffset<Player<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Player>>(SendGameScore::VT_PLAYER, player);
  }
  #[inline]
  pub fn add_game_result(&mut self, game_result: flatbuffers::WIPOffset<GameResult<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<GameResult>>(SendGameScore::VT_GAME_RESULT, game_result);
  }
  #[inline]
  pub fn add_score_message(&mut self, score_message: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SendGameScore::VT_SCORE_MESSAGE, score_message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SendGameScoreBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SendGameScoreBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SendGameScore<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_send_game_score<'a>(buf: &'a [u8]) -> SendGameScore<'a> {
  flatbuffers::get_root::<SendGameScore<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_send_game_score<'a>(buf: &'a [u8]) -> SendGameScore<'a> {
  flatbuffers::get_size_prefixed_root::<SendGameScore<'a>>(buf)
}

#[inline]
pub fn finish_send_game_score_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<SendGameScore<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_send_game_score_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<SendGameScore<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
