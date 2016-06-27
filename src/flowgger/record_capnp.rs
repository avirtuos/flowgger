// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: record.capnp


pub mod record {
  #![allow(unused_imports)]
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{text, data, Result};
    use capnp::private::layout;
    use capnp::traits::{FromStructBuilder, FromStructReader};
    use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

    pub struct Owned;
    impl<'a> ::capnp::traits::Owned<'a> for Owned {
        type Reader = Reader<'a>;
        type Builder = Builder<'a>;
    }
    impl<'a> ::capnp::traits::OwnedStruct<'a> for Owned {
        type Reader = Reader<'a>;
        type Builder = Builder<'a>;
    }
    impl ::capnp::traits::Pipelined for Owned {
        type Pipeline = Pipeline;
    }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> {
        reader: layout::StructReader<'a>,
    }

    impl<'a> ::capnp::traits::HasTypeId for Reader<'a> {
        #[inline]
        fn type_id() -> u64 {
            _private::TYPE_ID
        }
    }
    impl<'a> ::capnp::traits::FromStructReader<'a> for Reader<'a> {
        fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a> {
            Reader { reader: reader }
        }
    }

    impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
        fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>)
                            -> Result<Reader<'a>> {
            ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
        }
    }

    impl<'a> Reader<'a> {
        pub fn borrow<'b>(&'b self) -> Reader<'b> {
            Reader { ..*self }
        }

        pub fn total_size(&self) -> Result<::capnp::MessageSize> {
            self.reader.total_size()
        }
        #[inline]
        pub fn get_ts(self) -> i64 {
            self.reader.get_data_field::<i64>(0)
        }
        #[inline]
        pub fn get_hostname(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(0).get_text(::std::ptr::null(), 0)
        }
        pub fn has_hostname(&self) -> bool {
            !self.reader.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn get_facility(self) -> u8 {
            self.reader.get_data_field::<u8>(8)
        }
        #[inline]
        pub fn get_severity(self) -> u8 {
            self.reader.get_data_field::<u8>(9)
        }
        #[inline]
        pub fn get_appname(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(1).get_text(::std::ptr::null(), 0)
        }
        pub fn has_appname(&self) -> bool {
            !self.reader.get_pointer_field(1).is_null()
        }
        #[inline]
        pub fn get_procid(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(2).get_text(::std::ptr::null(), 0)
        }
        pub fn has_procid(&self) -> bool {
            !self.reader.get_pointer_field(2).is_null()
        }
        #[inline]
        pub fn get_msgid(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(3).get_text(::std::ptr::null(), 0)
        }
        pub fn has_msgid(&self) -> bool {
            !self.reader.get_pointer_field(3).is_null()
        }
        #[inline]
        pub fn get_msg(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(4).get_text(::std::ptr::null(), 0)
        }
        pub fn has_msg(&self) -> bool {
            !self.reader.get_pointer_field(4).is_null()
        }
        #[inline]
        pub fn get_full_msg(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(5).get_text(::std::ptr::null(), 0)
        }
        pub fn has_full_msg(&self) -> bool {
            !self.reader.get_pointer_field(5).is_null()
        }
        #[inline]
        pub fn get_sd_id(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(6).get_text(::std::ptr::null(), 0)
        }
        pub fn has_sd_id(&self) -> bool {
            !self.reader.get_pointer_field(6).is_null()
        }
        #[inline]
        pub fn get_pairs(self) -> Result<struct_list::Reader<'a, ::record_capnp::pair::Owned>> {
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(7))
        }
        pub fn has_pairs(&self) -> bool {
            !self.reader.get_pointer_field(7).is_null()
        }
    }

    pub struct Builder<'a> {
        builder: ::capnp::private::layout::StructBuilder<'a>,
    }
    impl<'a> ::capnp::traits::HasStructSize for Builder<'a> {
        #[inline]
        fn struct_size() -> layout::StructSize {
            _private::STRUCT_SIZE
        }
    }
    impl<'a> ::capnp::traits::HasTypeId for Builder<'a> {
        #[inline]
        fn type_id() -> u64 {
            _private::TYPE_ID
        }
    }
    impl<'a> ::capnp::traits::FromStructBuilder<'a> for Builder<'a> {
        fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a> {
            Builder { builder: builder }
        }
    }

    impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
        fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>,
                        _size: u32)
                        -> Builder<'a> {
            ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
        }
        fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>)
                            -> Result<Builder<'a>> {
            ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
        }
    }

    impl<'a> ::capnp::traits::SetPointerBuilder<Builder<'a>> for Reader<'a> {
        fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>,
                                   value: Reader<'a>)
                                   -> Result<()> {
            pointer.set_struct(&value.reader)
        }
    }

    impl<'a> Builder<'a> {
        pub fn as_reader(self) -> Reader<'a> {
            ::capnp::traits::FromStructReader::new(self.builder.as_reader())
        }
        pub fn borrow<'b>(&'b mut self) -> Builder<'b> {
            Builder { ..*self }
        }
        pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b> {
            ::capnp::traits::FromStructReader::new(self.builder.as_reader())
        }

        pub fn total_size(&self) -> Result<::capnp::MessageSize> {
            self.builder.as_reader().total_size()
        }
        #[inline]
        pub fn get_ts(self) -> i64 {
            self.builder.get_data_field::<i64>(0)
        }
        #[inline]
        pub fn set_ts(&mut self, value: i64) {
            self.builder.set_data_field::<i64>(0, value);
        }
        #[inline]
        pub fn get_hostname(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(0).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_hostname(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(0).set_text(value);
        }
        #[inline]
        pub fn init_hostname(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(0).init_text(size)
        }
        pub fn has_hostname(&self) -> bool {
            !self.builder.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn get_facility(self) -> u8 {
            self.builder.get_data_field::<u8>(8)
        }
        #[inline]
        pub fn set_facility(&mut self, value: u8) {
            self.builder.set_data_field::<u8>(8, value);
        }
        #[inline]
        pub fn get_severity(self) -> u8 {
            self.builder.get_data_field::<u8>(9)
        }
        #[inline]
        pub fn set_severity(&mut self, value: u8) {
            self.builder.set_data_field::<u8>(9, value);
        }
        #[inline]
        pub fn get_appname(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(1).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_appname(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(1).set_text(value);
        }
        #[inline]
        pub fn init_appname(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(1).init_text(size)
        }
        pub fn has_appname(&self) -> bool {
            !self.builder.get_pointer_field(1).is_null()
        }
        #[inline]
        pub fn get_procid(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(2).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_procid(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(2).set_text(value);
        }
        #[inline]
        pub fn init_procid(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(2).init_text(size)
        }
        pub fn has_procid(&self) -> bool {
            !self.builder.get_pointer_field(2).is_null()
        }
        #[inline]
        pub fn get_msgid(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(3).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_msgid(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(3).set_text(value);
        }
        #[inline]
        pub fn init_msgid(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(3).init_text(size)
        }
        pub fn has_msgid(&self) -> bool {
            !self.builder.get_pointer_field(3).is_null()
        }
        #[inline]
        pub fn get_msg(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(4).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_msg(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(4).set_text(value);
        }
        #[inline]
        pub fn init_msg(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(4).init_text(size)
        }
        pub fn has_msg(&self) -> bool {
            !self.builder.get_pointer_field(4).is_null()
        }
        #[inline]
        pub fn get_full_msg(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(5).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_full_msg(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(5).set_text(value);
        }
        #[inline]
        pub fn init_full_msg(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(5).init_text(size)
        }
        pub fn has_full_msg(&self) -> bool {
            !self.builder.get_pointer_field(5).is_null()
        }
        #[inline]
        pub fn get_sd_id(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(6).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_sd_id(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(6).set_text(value);
        }
        #[inline]
        pub fn init_sd_id(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(6).init_text(size)
        }
        pub fn has_sd_id(&self) -> bool {
            !self.builder.get_pointer_field(6).is_null()
        }
        #[inline]
        pub fn get_pairs(self) -> Result<struct_list::Builder<'a, ::record_capnp::pair::Owned>> {
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(7))
        }
        #[inline]
        pub fn set_pairs(&mut self,
                         value: struct_list::Reader<'a, ::record_capnp::pair::Owned>)
                         -> Result<()> {
            ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder
                                                                        .get_pointer_field(7),
                                                                    value)
        }
        #[inline]
        pub fn init_pairs(self,
                          size: u32)
                          -> struct_list::Builder<'a, ::record_capnp::pair::Owned> {
            ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(7),
                                                              size)
        }
        pub fn has_pairs(&self) -> bool {
            !self.builder.get_pointer_field(7).is_null()
        }
    }

    pub struct Pipeline {
        _typeless: ::capnp::any_pointer::Pipeline,
    }
    impl FromTypelessPipeline for Pipeline {
        fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
            Pipeline { _typeless: typeless }
        }
    }
    impl Pipeline {}
    mod _private {
        use capnp::private::layout;
        pub const STRUCT_SIZE: layout::StructSize = layout::StructSize {
            data: 2,
            pointers: 8,
        };
        pub const TYPE_ID: u64 = 0xff99ba646e037d20;
    }
}

pub mod pair {
  #![allow(unused_imports)]
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{text, data, Result};
    use capnp::private::layout;
    use capnp::traits::{FromStructBuilder, FromStructReader};
    use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

    pub struct Owned;
    impl<'a> ::capnp::traits::Owned<'a> for Owned {
        type Reader = Reader<'a>;
        type Builder = Builder<'a>;
    }
    impl<'a> ::capnp::traits::OwnedStruct<'a> for Owned {
        type Reader = Reader<'a>;
        type Builder = Builder<'a>;
    }
    impl ::capnp::traits::Pipelined for Owned {
        type Pipeline = Pipeline;
    }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> {
        reader: layout::StructReader<'a>,
    }

    impl<'a> ::capnp::traits::HasTypeId for Reader<'a> {
        #[inline]
        fn type_id() -> u64 {
            _private::TYPE_ID
        }
    }
    impl<'a> ::capnp::traits::FromStructReader<'a> for Reader<'a> {
        fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a> {
            Reader { reader: reader }
        }
    }

    impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
        fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>)
                            -> Result<Reader<'a>> {
            ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
        }
    }

    impl<'a> Reader<'a> {
        pub fn borrow<'b>(&'b self) -> Reader<'b> {
            Reader { ..*self }
        }

        pub fn total_size(&self) -> Result<::capnp::MessageSize> {
            self.reader.total_size()
        }
        #[inline]
        pub fn get_key(self) -> Result<text::Reader<'a>> {
            self.reader.get_pointer_field(0).get_text(::std::ptr::null(), 0)
        }
        pub fn has_key(&self) -> bool {
            !self.reader.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn get_value(self) -> ::record_capnp::pair::value::Reader<'a> {
            ::capnp::traits::FromStructReader::new(self.reader)
        }
    }

    pub struct Builder<'a> {
        builder: ::capnp::private::layout::StructBuilder<'a>,
    }
    impl<'a> ::capnp::traits::HasStructSize for Builder<'a> {
        #[inline]
        fn struct_size() -> layout::StructSize {
            _private::STRUCT_SIZE
        }
    }
    impl<'a> ::capnp::traits::HasTypeId for Builder<'a> {
        #[inline]
        fn type_id() -> u64 {
            _private::TYPE_ID
        }
    }
    impl<'a> ::capnp::traits::FromStructBuilder<'a> for Builder<'a> {
        fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a> {
            Builder { builder: builder }
        }
    }

    impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
        fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>,
                        _size: u32)
                        -> Builder<'a> {
            ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
        }
        fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>)
                            -> Result<Builder<'a>> {
            ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
        }
    }

    impl<'a> ::capnp::traits::SetPointerBuilder<Builder<'a>> for Reader<'a> {
        fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>,
                                   value: Reader<'a>)
                                   -> Result<()> {
            pointer.set_struct(&value.reader)
        }
    }

    impl<'a> Builder<'a> {
        pub fn as_reader(self) -> Reader<'a> {
            ::capnp::traits::FromStructReader::new(self.builder.as_reader())
        }
        pub fn borrow<'b>(&'b mut self) -> Builder<'b> {
            Builder { ..*self }
        }
        pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b> {
            ::capnp::traits::FromStructReader::new(self.builder.as_reader())
        }

        pub fn total_size(&self) -> Result<::capnp::MessageSize> {
            self.builder.as_reader().total_size()
        }
        #[inline]
        pub fn get_key(self) -> Result<text::Builder<'a>> {
            self.builder.get_pointer_field(0).get_text(::std::ptr::null(), 0)
        }
        #[inline]
        pub fn set_key(&mut self, value: text::Reader) {
            self.builder.get_pointer_field(0).set_text(value);
        }
        #[inline]
        pub fn init_key(self, size: u32) -> text::Builder<'a> {
            self.builder.get_pointer_field(0).init_text(size)
        }
        pub fn has_key(&self) -> bool {
            !self.builder.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn get_value(self) -> ::record_capnp::pair::value::Builder<'a> {
            ::capnp::traits::FromStructBuilder::new(self.builder)
        }
        #[inline]
        pub fn init_value(self) -> ::record_capnp::pair::value::Builder<'a> {
            self.builder.set_data_field::<u16>(0, 0);
            self.builder.get_pointer_field(1).clear();
            self.builder.set_bool_field(16, false);
            self.builder.set_data_field::<f64>(1, 0u8 as f64);
            self.builder.set_data_field::<i64>(1, 0u8 as i64);
            self.builder.set_data_field::<u64>(1, 0u8 as u64);
            ::capnp::traits::FromStructBuilder::new(self.builder)
        }
    }

    pub struct Pipeline {
        _typeless: ::capnp::any_pointer::Pipeline,
    }
    impl FromTypelessPipeline for Pipeline {
        fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
            Pipeline { _typeless: typeless }
        }
    }
    impl Pipeline {
        pub fn get_value(&self) -> ::record_capnp::pair::value::Pipeline {
            FromTypelessPipeline::new(self._typeless.noop())
        }
    }
    mod _private {
        use capnp::private::layout;
        pub const STRUCT_SIZE: layout::StructSize = layout::StructSize {
            data: 2,
            pointers: 2,
        };
        pub const TYPE_ID: u64 = 0xf3980b784d305367;
    }

    pub mod value {
    #![allow(unused_imports)]
        use capnp::capability::{FromClientHook, FromTypelessPipeline};
        use capnp::{text, data, Result};
        use capnp::private::layout;
        use capnp::traits::{FromStructBuilder, FromStructReader};
        use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

        pub use self::Which::{String, Bool, F64, I64, U64, Null};

        pub struct Owned;
        impl<'a> ::capnp::traits::Owned<'a> for Owned {
            type Reader = Reader<'a>;
            type Builder = Builder<'a>;
        }
        impl<'a> ::capnp::traits::OwnedStruct<'a> for Owned {
            type Reader = Reader<'a>;
            type Builder = Builder<'a>;
        }
        impl ::capnp::traits::Pipelined for Owned {
            type Pipeline = Pipeline;
        }

        #[derive(Clone, Copy)]
        pub struct Reader<'a> {
            reader: layout::StructReader<'a>,
        }

        impl<'a> ::capnp::traits::HasTypeId for Reader<'a> {
            #[inline]
            fn type_id() -> u64 {
                _private::TYPE_ID
            }
        }
        impl<'a> ::capnp::traits::FromStructReader<'a> for Reader<'a> {
            fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a> {
                Reader { reader: reader }
            }
        }

        impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
            fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>)
                                -> Result<Reader<'a>> {
                ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
            }
        }

        impl<'a> Reader<'a> {
            pub fn borrow<'b>(&'b self) -> Reader<'b> {
                Reader { ..*self }
            }

            pub fn total_size(&self) -> Result<::capnp::MessageSize> {
                self.reader.total_size()
            }
            pub fn has_string(&self) -> bool {
                if self.reader.get_data_field::<u16>(0) != 0 {
                    return false;
                }
                !self.reader.get_pointer_field(1).is_null()
            }
            #[inline]
            pub fn which(self) -> ::std::result::Result<WhichReader<'a>, ::capnp::NotInSchema> {
                match self.reader.get_data_field::<u16>(0) {
                    0 => {
                        return ::std::result::Result::Ok(String(self.reader
                            .get_pointer_field(1)
                            .get_text(::std::ptr::null(), 0)));
                    }
                    1 => {
                        return ::std::result::Result::Ok(Bool(self.reader.get_bool_field(16)));
                    }
                    2 => {
                        return ::std::result::Result::Ok(F64(self.reader.get_data_field::<f64>(1)));
                    }
                    3 => {
                        return ::std::result::Result::Ok(I64(self.reader.get_data_field::<i64>(1)));
                    }
                    4 => {
                        return ::std::result::Result::Ok(U64(self.reader.get_data_field::<u64>(1)));
                    }
                    5 => {
                        return ::std::result::Result::Ok(Null(()));
                    }
                    x => return ::std::result::Result::Err(::capnp::NotInSchema(x)),
                }
            }
        }

        pub struct Builder<'a> {
            builder: ::capnp::private::layout::StructBuilder<'a>,
        }
        impl<'a> ::capnp::traits::HasStructSize for Builder<'a> {
            #[inline]
            fn struct_size() -> layout::StructSize {
                _private::STRUCT_SIZE
            }
        }
        impl<'a> ::capnp::traits::HasTypeId for Builder<'a> {
            #[inline]
            fn type_id() -> u64 {
                _private::TYPE_ID
            }
        }
        impl<'a> ::capnp::traits::FromStructBuilder<'a> for Builder<'a> {
            fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a> {
                Builder { builder: builder }
            }
        }

        impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
            fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>,
                            _size: u32)
                            -> Builder<'a> {
                ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
            }
            fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>)
                                -> Result<Builder<'a>> {
                ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
            }
        }

        impl<'a> ::capnp::traits::SetPointerBuilder<Builder<'a>> for Reader<'a> {
            fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>,
                                       value: Reader<'a>)
                                       -> Result<()> {
                pointer.set_struct(&value.reader)
            }
        }

        impl<'a> Builder<'a> {
            pub fn as_reader(self) -> Reader<'a> {
                ::capnp::traits::FromStructReader::new(self.builder.as_reader())
            }
            pub fn borrow<'b>(&'b mut self) -> Builder<'b> {
                Builder { ..*self }
            }
            pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b> {
                ::capnp::traits::FromStructReader::new(self.builder.as_reader())
            }

            pub fn total_size(&self) -> Result<::capnp::MessageSize> {
                self.builder.as_reader().total_size()
            }
            #[inline]
            pub fn set_string(&mut self, value: text::Reader) {
                self.builder.set_data_field::<u16>(0, 0);
                self.builder.get_pointer_field(1).set_text(value);
            }
            #[inline]
            pub fn init_string(self, size: u32) -> text::Builder<'a> {
                self.builder.set_data_field::<u16>(0, 0);
                self.builder.get_pointer_field(1).init_text(size)
            }
            pub fn has_string(&self) -> bool {
                if self.builder.get_data_field::<u16>(0) != 0 {
                    return false;
                }
                !self.builder.get_pointer_field(1).is_null()
            }
            #[inline]
            pub fn set_bool(&mut self, value: bool) {
                self.builder.set_data_field::<u16>(0, 1);
                self.builder.set_bool_field(16, value);
            }
            #[inline]
            pub fn set_f64(&mut self, value: f64) {
                self.builder.set_data_field::<u16>(0, 2);
                self.builder.set_data_field::<f64>(1, value);
            }
            #[inline]
            pub fn set_i64(&mut self, value: i64) {
                self.builder.set_data_field::<u16>(0, 3);
                self.builder.set_data_field::<i64>(1, value);
            }
            #[inline]
            pub fn set_u64(&mut self, value: u64) {
                self.builder.set_data_field::<u16>(0, 4);
                self.builder.set_data_field::<u64>(1, value);
            }
            #[inline]
            pub fn set_null(&mut self, _value: ()) {
                self.builder.set_data_field::<u16>(0, 5);
            }
            #[inline]
            pub fn which(self) -> ::std::result::Result<WhichBuilder<'a>, ::capnp::NotInSchema> {
                match self.builder.get_data_field::<u16>(0) {
                    0 => {
                        return ::std::result::Result::Ok(String(self.builder
                            .get_pointer_field(1)
                            .get_text(::std::ptr::null(), 0)));
                    }
                    1 => {
                        return ::std::result::Result::Ok(Bool(self.builder.get_bool_field(16)));
                    }
                    2 => {
                        return ::std::result::Result::Ok(F64(self.builder
                            .get_data_field::<f64>(1)));
                    }
                    3 => {
                        return ::std::result::Result::Ok(I64(self.builder
                            .get_data_field::<i64>(1)));
                    }
                    4 => {
                        return ::std::result::Result::Ok(U64(self.builder
                            .get_data_field::<u64>(1)));
                    }
                    5 => {
                        return ::std::result::Result::Ok(Null(()));
                    }
                    x => return ::std::result::Result::Err(::capnp::NotInSchema(x)),
                }
            }
        }

        pub struct Pipeline {
            _typeless: ::capnp::any_pointer::Pipeline,
        }
        impl FromTypelessPipeline for Pipeline {
            fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
                Pipeline { _typeless: typeless }
            }
        }
        impl Pipeline {}
        mod _private {
            use capnp::private::layout;
            pub const STRUCT_SIZE: layout::StructSize = layout::StructSize {
                data: 2,
                pointers: 2,
            };
            pub const TYPE_ID: u64 = 0xc960eefdd10a29b3;
        }
        pub enum Which<A0> {
            String(A0),
            Bool(bool),
            F64(f64),
            I64(i64),
            U64(u64),
            Null(()),
        }
        pub type WhichReader<'a> = Which<Result<text::Reader<'a>>>;
        pub type WhichBuilder<'a> = Which<Result<text::Builder<'a>>>;
    }
}
