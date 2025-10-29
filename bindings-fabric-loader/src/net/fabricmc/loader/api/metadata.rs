pub mod version;
///interface [ContactInformation](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ContactInformation.html)
pub enum ContactInformation {}
unsafe impl ::java_spaghetti::ReferenceType for ContactInformation {}
unsafe impl ::java_spaghetti::JniType for ContactInformation {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ContactInformation")
    }
}
impl ContactInformation {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ContactInformation"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///**get** public static final [EMPTY](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ContactInformation.html#EMPTY)
    pub fn EMPTY<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ContactInformation>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"EMPTY",
                        c"Lnet/fabricmc/loader/api/metadata/ContactInformation;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///class [ContactInformation.1](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ContactInformation.1.html)
enum ContactInformation__1 {}
unsafe impl ::java_spaghetti::ReferenceType for ContactInformation__1 {}
unsafe impl ::java_spaghetti::JniType for ContactInformation__1 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ContactInformation$1")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<ContactInformation> for ContactInformation__1 {}
impl ContactInformation__1 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/ContactInformation$1"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///interface [CustomValue](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.html)
pub enum CustomValue {}
unsafe impl ::java_spaghetti::ReferenceType for CustomValue {}
unsafe impl ::java_spaghetti::JniType for CustomValue {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue")
    }
}
impl CustomValue {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/CustomValue"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.html#getType())
    pub fn getType<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getType",
                        c"()Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsObject](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.html#getAsObject())
    pub fn getAsObject<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvObject>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getAsObject",
                        c"()Lnet/fabricmc/loader/api/metadata/CustomValue$CvObject;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsArray](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.html#getAsArray())
    pub fn getAsArray<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvArray>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getAsArray",
                        c"()Lnet/fabricmc/loader/api/metadata/CustomValue$CvArray;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAsBoolean](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.html#getAsBoolean())
    pub fn getAsBoolean<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getAsBoolean",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [CustomValue.CvArray](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvArray.html)
pub enum CustomValue_CvArray {}
unsafe impl ::java_spaghetti::ReferenceType for CustomValue_CvArray {}
unsafe impl ::java_spaghetti::JniType for CustomValue_CvArray {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue$CvArray")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<CustomValue> for CustomValue_CvArray {}
impl CustomValue_CvArray {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/CustomValue$CvArray"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[size](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvArray.html#size())
    pub fn size<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        i32,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"size",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[get](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvArray.html#get(int))
    pub fn get<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, CustomValue>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"get",
                        c"(I)Lnet/fabricmc/loader/api/metadata/CustomValue;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [CustomValue.CvObject](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvObject.html)
pub enum CustomValue_CvObject {}
unsafe impl ::java_spaghetti::ReferenceType for CustomValue_CvObject {}
unsafe impl ::java_spaghetti::JniType for CustomValue_CvObject {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue$CvObject")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<CustomValue> for CustomValue_CvObject {}
impl CustomValue_CvObject {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/CustomValue$CvObject"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[size](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvObject.html#size())
    pub fn size<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        i32,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"size",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [CustomValue.CvType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html)
pub enum CustomValue_CvType {}
unsafe impl ::java_spaghetti::ReferenceType for CustomValue_CvType {}
unsafe impl ::java_spaghetti::JniType for CustomValue_CvType {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/CustomValue$CvType")
    }
}
impl CustomValue_CvType {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/CustomValue$CvType"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#values())
    pub fn values<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<
                    CustomValue_CvType,
                    super::super::super::super::super::java::lang::Throwable,
                >,
            >,
        >,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"values",
                        c"()[Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [OBJECT](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#OBJECT)
    pub fn OBJECT<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"OBJECT",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [ARRAY](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#ARRAY)
    pub fn ARRAY<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"ARRAY",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [STRING](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#STRING)
    pub fn STRING<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"STRING",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [NUMBER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#NUMBER)
    pub fn NUMBER<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"NUMBER",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [BOOLEAN](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#BOOLEAN)
    pub fn BOOLEAN<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"BOOLEAN",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [NULL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/CustomValue.CvType.html#NULL)
    pub fn NULL<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, CustomValue_CvType>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"NULL",
                        c"Lnet/fabricmc/loader/api/metadata/CustomValue$CvType;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [ModDependency](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.html)
pub enum ModDependency {}
unsafe impl ::java_spaghetti::ReferenceType for ModDependency {}
unsafe impl ::java_spaghetti::JniType for ModDependency {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModDependency")
    }
}
impl ModDependency {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModDependency"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getKind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.html#getKind())
    pub fn getKind<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, ModDependency_Kind>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getKind",
                        c"()Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[matches](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.html#matches(net.fabricmc.loader.api.Version))
    pub fn matches<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::Version>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"matches",
                        c"(Lnet/fabricmc/loader/api/Version;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [ModDependency.Kind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html)
pub enum ModDependency_Kind {}
unsafe impl ::java_spaghetti::ReferenceType for ModDependency_Kind {}
unsafe impl ::java_spaghetti::JniType for ModDependency_Kind {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModDependency$Kind")
    }
}
impl ModDependency_Kind {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModDependency$Kind"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#values())
    pub fn values<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<
                    ModDependency_Kind,
                    super::super::super::super::super::java::lang::Throwable,
                >,
            >,
        >,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"values",
                        c"()[Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isPositive](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#isPositive())
    pub fn isPositive<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"isPositive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isSoft](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#isSoft())
    pub fn isSoft<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"isSoft",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [DEPENDS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#DEPENDS)
    pub fn DEPENDS<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"DEPENDS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [RECOMMENDS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#RECOMMENDS)
    pub fn RECOMMENDS<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"RECOMMENDS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SUGGESTS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#SUGGESTS)
    pub fn SUGGESTS<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SUGGESTS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [CONFLICTS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#CONFLICTS)
    pub fn CONFLICTS<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"CONFLICTS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [BREAKS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModDependency.Kind.html#BREAKS)
    pub fn BREAKS<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModDependency_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"BREAKS",
                        c"Lnet/fabricmc/loader/api/metadata/ModDependency$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///enum [ModEnvironment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModEnvironment.html)
pub enum ModEnvironment {}
unsafe impl ::java_spaghetti::ReferenceType for ModEnvironment {}
unsafe impl ::java_spaghetti::JniType for ModEnvironment {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModEnvironment")
    }
}
impl ModEnvironment {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModEnvironment"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModEnvironment.html#values())
    pub fn values<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<
                    ModEnvironment,
                    super::super::super::super::super::java::lang::Throwable,
                >,
            >,
        >,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"values",
                        c"()[Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[matches](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModEnvironment.html#matches(net.fabricmc.api.EnvType))
    pub fn matches<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::super::api::EnvType>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"matches",
                        c"(Lnet/fabricmc/api/EnvType;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [CLIENT](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModEnvironment.html#CLIENT)
    pub fn CLIENT<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModEnvironment>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"CLIENT",
                        c"Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SERVER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModEnvironment.html#SERVER)
    pub fn SERVER<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModEnvironment>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SERVER",
                        c"Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [UNIVERSAL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModEnvironment.html#UNIVERSAL)
    pub fn UNIVERSAL<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModEnvironment>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"UNIVERSAL",
                        c"Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [ModMetadata](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModMetadata.html)
pub enum ModMetadata {}
unsafe impl ::java_spaghetti::ReferenceType for ModMetadata {}
unsafe impl ::java_spaghetti::JniType for ModMetadata {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModMetadata")
    }
}
impl ModMetadata {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModMetadata"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModMetadata.html#getVersion())
    pub fn getVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::Version>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getVersion",
                        c"()Lnet/fabricmc/loader/api/Version;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getEnvironment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModMetadata.html#getEnvironment())
    pub fn getEnvironment<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, ModEnvironment>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getEnvironment",
                        c"()Lnet/fabricmc/loader/api/metadata/ModEnvironment;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getContact](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModMetadata.html#getContact())
    pub fn getContact<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, ContactInformation>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getContact",
                        c"()Lnet/fabricmc/loader/api/metadata/ContactInformation;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ModOrigin](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.html)
pub enum ModOrigin {}
unsafe impl ::java_spaghetti::ReferenceType for ModOrigin {}
unsafe impl ::java_spaghetti::JniType for ModOrigin {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModOrigin")
    }
}
impl ModOrigin {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModOrigin"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getKind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.html#getKind())
    pub fn getKind<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, ModOrigin_Kind>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getKind",
                        c"()Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [ModOrigin.Kind](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html)
pub enum ModOrigin_Kind {}
unsafe impl ::java_spaghetti::ReferenceType for ModOrigin_Kind {}
unsafe impl ::java_spaghetti::JniType for ModOrigin_Kind {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/ModOrigin$Kind")
    }
}
impl ModOrigin_Kind {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/ModOrigin$Kind"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#values())
    pub fn values<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<
                    ModOrigin_Kind,
                    super::super::super::super::super::java::lang::Throwable,
                >,
            >,
        >,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"values",
                        c"()[Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [PATH](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#PATH)
    pub fn PATH<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModOrigin_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"PATH",
                        c"Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [NESTED](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#NESTED)
    pub fn NESTED<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModOrigin_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"NESTED",
                        c"Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [UNKNOWN](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/ModOrigin.Kind.html#UNKNOWN)
    pub fn UNKNOWN<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, ModOrigin_Kind>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"UNKNOWN",
                        c"Lnet/fabricmc/loader/api/metadata/ModOrigin$Kind;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [Person](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/Person.html)
pub enum Person {}
unsafe impl ::java_spaghetti::ReferenceType for Person {}
unsafe impl ::java_spaghetti::JniType for Person {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/Person")
    }
}
impl Person {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/metadata/Person"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getContact](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/Person.html#getContact())
    pub fn getContact<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, ContactInformation>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getContact",
                        c"()Lnet/fabricmc/loader/api/metadata/ContactInformation;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
