pub mod class_ext;
pub mod state;

use gdnative::prelude::*;

// Benchmark synchronous blocks of code in godot environment
#[macro_export]
macro_rules! log_time {
  ($name: expr, $block: block) => {
    let os = gdnative::api::OS::godot_singleton();
    let start = os.get_ticks_usec();
    $block;
    godot_print!("#time {:?}: {}", $name, os.get_ticks_usec() - start);
  };
}

// Unwrap Option<Ref<T>> -> Option<TRef<T>>
#[macro_export]
macro_rules! unwrap_ref {
  ($ref:expr) => {
    unsafe { $ref.map(|n| n.assume_safe()) }
  };
}

pub fn call_fn_opt(node: Option<TRef<Node>>, fn_name: &str, args: &[Variant]) -> Option<Variant> {
  node.and_then(|n| {
    if n.has_method(fn_name) {
      Some(unsafe { n.call(fn_name, args) })
    } else {
      None
    }
  })
}

pub fn get_node<T>(owner: impl NodeExt, selector: &str) -> Option<Ref<T>>
where
  T: GodotObject + SubClass<Node>,
{
  let node = unsafe { owner.get_node_as::<T>(selector) };
  node.map(|n| n.claim())
}
