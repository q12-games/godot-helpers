#[macro_export]
macro_rules! from_variant {
  ((), $val: expr) => {
    Some(())
  };
  ($ret: ty, $val: expr) => {
    <$ret>::from_variant($val)
  };
}

#[macro_export]
macro_rules! singleton_methods {
  (
    class $class:ident;
    name $node_name:expr;

    $(
      fn $fn_name:ident (
        & $($self_declr:ident)+,
        $owner:ident : $owner_ty:ty
        $(, $arg:ident : $arg_ty:ty)*
      ) -> $return:ty
        $body:block
    )*
  ) => {
    fn get_node(owner: &Node) -> Option<gdnative::TRef<gdnative::api::Node>> {
      unsafe { owner.get_node_as::<gdnative::api::Node>($node_name) }
    }

    paste::paste! {
      fn register_singleton_methods(builder: &gdnative::nativescript::ClassBuilder<Self>) {
        $(
          builder.add_method(
            stringify!([<__ $fn_name>]),
            gdnative::godot_wrap_method!(
              $class,
              fn [<__ $fn_name>](& $($self_declr)+, $owner: $owner_ty $(, $arg: $arg_ty)*) -> $return
            )
          );
        )*
      }

      $(
        pub fn $fn_name(node_ref: &gdnative::api::Node $(, $arg: $arg_ty)*) -> $return {
          q12_godot_helpers::call_fn_opt(Self::get_node(node_ref), stringify!([<__ $fn_name>]), &[
            $($arg.to_variant(),)*
          ])
            .map(|v| q12_godot_helpers::from_variant!($return, &v)
              .expect(&format!("[{}] Unable to resolve return type", $node_name)))
            .expect(&format!("[{}] Unable to call method on node", $node_name))
        }

        fn [<__ $fn_name>](& $($self_declr)+, $owner: $owner_ty $(, $arg: $arg_ty)*) -> $return
          $body
      )*
    }
  };
}

#[macro_export]
macro_rules! connect_signals {
  (
    class $class:ident;
    owner $owner:ident;

    $(
      $(@[$target:expr].on($ev:expr))*
      $(#[$struct_meta:meta])*
      fn $fn_name:ident(& $($self_declr: ident)+ $(, $arg:ident: $ty:ty)*) $body: block
    )*
  ) => {
    fn connect_signals(owner: gdnative::TRef<$owner>) {
      // TODO: Maybe call this on the owner's ready signal instead of manual call inside function?
      $(
        $(
          let target: gdnative::TRef<gdnative::api::Node> = match $target {
            "." => Some(owner.upcast::<gdnative::api::Node>()),
            ".." => owner.get_parent().map(|n| unsafe { n.assume_safe() }),
            selector => unsafe { owner.as_ref().get_node_as::<gdnative::api::Node>(selector) },
          }.expect(&format!("Couldn't find {}", $target));

          let args = gdnative::core_types::VariantArray::new();
          args.push(target);

          target
            .connect($ev, owner, stringify!($fn_name), args.into_shared(), 0)
            .unwrap();
        )*
      )*
    }

    fn register_signals(builder: &gdnative::nativescript::ClassBuilder<Self>) {
      $(
        builder.add_method(
          stringify!($fn_name),
          gdnative::godot_wrap_method!(
            $class,
            fn $fn_name(& $($self_declr)+ $(, $arg: $ty)*) -> ()
          ),
        );
      )*
    }

    $(
      $(#[$struct_meta])*
      fn $fn_name(& $($self_declr)+ $(, $arg: $ty)*) $body
    )*
  };
}
