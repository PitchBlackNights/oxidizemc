///enum [VersionComparisonOperator](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html)
pub enum VersionComparisonOperator {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator")
    }
}
impl VersionComparisonOperator {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[values](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#values())
    pub fn values<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<
            ::java_spaghetti::Local<
                'env,
                ::java_spaghetti::ObjectArray<
                    VersionComparisonOperator,
                    super::super::super::super::super::super::java::lang::Throwable,
                >,
            >,
        >,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"()[Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isMinInclusive](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#isMinInclusive())
    pub fn isMinInclusive<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"isMinInclusive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isMaxInclusive](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#isMaxInclusive())
    pub fn isMaxInclusive<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"isMaxInclusive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#test(net.fabricmc.loader.api.Version,net.fabricmc.loader.api.Version))
    pub fn test_Version_Version<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::Version>,
        arg1: impl ::java_spaghetti::AsArg<super::super::Version>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"test",
                        c"(Lnet/fabricmc/loader/api/Version;Lnet/fabricmc/loader/api/Version;)Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test_SemanticVersion_SemanticVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[minVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#minVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn minVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"minVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[maxVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#maxVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn maxVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"maxVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [GREATER_EQUAL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#GREATER_EQUAL)
    pub fn GREATER_EQUAL<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"GREATER_EQUAL",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [LESS_EQUAL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#LESS_EQUAL)
    pub fn LESS_EQUAL<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"LESS_EQUAL",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [GREATER](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#GREATER)
    pub fn GREATER<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"GREATER",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [LESS](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#LESS)
    pub fn LESS<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"LESS",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [EQUAL](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#EQUAL)
    pub fn EQUAL<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"EQUAL",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SAME_TO_NEXT_MINOR](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#SAME_TO_NEXT_MINOR)
    pub fn SAME_TO_NEXT_MINOR<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SAME_TO_NEXT_MINOR",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
    ///**get** public static final [SAME_TO_NEXT_MAJOR](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.html#SAME_TO_NEXT_MAJOR)
    pub fn SAME_TO_NEXT_MAJOR<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"SAME_TO_NEXT_MAJOR",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///enum [VersionComparisonOperator.1](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.1.html)
enum VersionComparisonOperator__1 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__1 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__1 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$1")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__1
{
}
impl VersionComparisonOperator__1 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$1",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.1.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[minVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.1.html#minVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn minVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"minVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [VersionComparisonOperator.2](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.2.html)
enum VersionComparisonOperator__2 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__2 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__2 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$2")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__2
{
}
impl VersionComparisonOperator__2 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$2",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.2.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[maxVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.2.html#maxVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn maxVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"maxVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [VersionComparisonOperator.3](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.3.html)
enum VersionComparisonOperator__3 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__3 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__3 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$3")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__3
{
}
impl VersionComparisonOperator__3 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$3",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.3.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[minVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.3.html#minVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn minVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"minVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [VersionComparisonOperator.4](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.4.html)
enum VersionComparisonOperator__4 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__4 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__4 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$4")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__4
{
}
impl VersionComparisonOperator__4 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$4",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.4.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[maxVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.4.html#maxVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn maxVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"maxVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [VersionComparisonOperator.5](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.5.html)
enum VersionComparisonOperator__5 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__5 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__5 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$5")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__5
{
}
impl VersionComparisonOperator__5 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$5",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.5.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[minVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.5.html#minVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn minVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"minVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[maxVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.5.html#maxVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn maxVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"maxVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [VersionComparisonOperator.6](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.6.html)
enum VersionComparisonOperator__6 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__6 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__6 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$6")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__6
{
}
impl VersionComparisonOperator__6 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$6",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.6.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[minVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.6.html#minVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn minVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"minVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[maxVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.6.html#maxVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn maxVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"maxVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///enum [VersionComparisonOperator.7](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.7.html)
enum VersionComparisonOperator__7 {}
unsafe impl ::java_spaghetti::ReferenceType for VersionComparisonOperator__7 {}
unsafe impl ::java_spaghetti::JniType for VersionComparisonOperator__7 {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$7")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<VersionComparisonOperator>
    for VersionComparisonOperator__7
{
}
impl VersionComparisonOperator__7 {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionComparisonOperator$7",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[test](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.7.html#test(net.fabricmc.loader.api.SemanticVersion,net.fabricmc.loader.api.SemanticVersion))
    pub fn test<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
        arg1: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"test" , c"(Lnet/fabricmc/loader/api/SemanticVersion;Lnet/fabricmc/loader/api/SemanticVersion;)Z"))) . as_raw () ;
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[minVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.7.html#minVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn minVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"minVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[maxVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionComparisonOperator.7.html#maxVersion(net.fabricmc.loader.api.SemanticVersion))
    pub fn maxVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<super::super::SemanticVersion>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::SemanticVersion>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"maxVersion" , c"(Lnet/fabricmc/loader/api/SemanticVersion;)Lnet/fabricmc/loader/api/SemanticVersion;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [VersionInterval](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html)
pub enum VersionInterval {}
unsafe impl ::java_spaghetti::ReferenceType for VersionInterval {}
unsafe impl ::java_spaghetti::JniType for VersionInterval {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionInterval")
    }
}
impl VersionInterval {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/metadata/version/VersionInterval"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[isSemantic](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#isSemantic())
    pub fn isSemantic<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"isSemantic",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getMin](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#getMin())
    pub fn getMin<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::Version>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"getMin",
                        c"()Lnet/fabricmc/loader/api/Version;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isMinInclusive](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#isMinInclusive())
    pub fn isMinInclusive<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"isMinInclusive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getMax](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#getMax())
    pub fn getMax<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::Version>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"getMax",
                        c"()Lnet/fabricmc/loader/api/Version;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isMaxInclusive](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#isMaxInclusive())
    pub fn isMaxInclusive<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"isMaxInclusive",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[and](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#and(net.fabricmc.loader.api.metadata.version.VersionInterval))
    pub fn and_VersionInterval<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<VersionInterval>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, VersionInterval>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_spaghetti::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_method (__jni_class , c"and" , c"(Lnet/fabricmc/loader/api/metadata/version/VersionInterval;)Lnet/fabricmc/loader/api/metadata/version/VersionInterval;"))) . as_raw () ;
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[and](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#and(net.fabricmc.loader.api.metadata.version.VersionInterval,net.fabricmc.loader.api.metadata.version.VersionInterval))
    pub fn and_VersionInterval_VersionInterval<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
        arg0: impl ::java_spaghetti::AsArg<VersionInterval>,
        arg1: impl ::java_spaghetti::AsArg<VersionInterval>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, VersionInterval>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [
                ::java_spaghetti::AsJValue::as_jvalue(&arg0),
                ::java_spaghetti::AsJValue::as_jvalue(&arg1),
            ];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD . get_or_init (| | :: java_spaghetti :: JMethodID :: from_raw (__jni_env . require_static_method (__jni_class , c"and" , c"(Lnet/fabricmc/loader/api/metadata/version/VersionInterval;Lnet/fabricmc/loader/api/metadata/version/VersionInterval;)Lnet/fabricmc/loader/api/metadata/version/VersionInterval;"))) . as_raw () ;
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///**get** public static final [INFINITE](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionInterval.html#INFINITE)
    pub fn INFINITE<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::option::Option<::java_spaghetti::Local<'env, VersionInterval>> {
        static __FIELD: ::std::sync::OnceLock<::java_spaghetti::JFieldID> =
            ::std::sync::OnceLock::new();
        let __jni_class = Self::__class_global_ref(__jni_env);
        unsafe {
            let __jni_field = __FIELD
                .get_or_init(|| {
                    ::java_spaghetti::JFieldID::from_raw(__jni_env.require_static_field(
                        __jni_class,
                        c"INFINITE",
                        c"Lnet/fabricmc/loader/api/metadata/version/VersionInterval;",
                    ))
                })
                .as_raw();
            __jni_env.get_static_object_field(__jni_class, __jni_field)
        }
    }
}
///interface [VersionPredicate](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionPredicate.html)
pub enum VersionPredicate {}
unsafe impl ::java_spaghetti::ReferenceType for VersionPredicate {}
unsafe impl ::java_spaghetti::JniType for VersionPredicate {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionPredicate")
    }
}
impl VersionPredicate {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionPredicate",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getInterval](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionPredicate.html#getInterval())
    pub fn getInterval<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, VersionInterval>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"getInterval",
                        c"()Lnet/fabricmc/loader/api/metadata/version/VersionInterval;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [VersionPredicate.PredicateTerm](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionPredicate.PredicateTerm.html)
pub enum VersionPredicate_PredicateTerm {}
unsafe impl ::java_spaghetti::ReferenceType for VersionPredicate_PredicateTerm {}
unsafe impl ::java_spaghetti::JniType for VersionPredicate_PredicateTerm {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/metadata/version/VersionPredicate$PredicateTerm")
    }
}
impl VersionPredicate_PredicateTerm {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(
                        c"net/fabricmc/loader/api/metadata/version/VersionPredicate$PredicateTerm",
                    ),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getOperator](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionPredicate.PredicateTerm.html#getOperator())
    pub fn getOperator<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, VersionComparisonOperator>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"getOperator",
                        c"()Lnet/fabricmc/loader/api/metadata/version/VersionComparisonOperator;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getReferenceVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/metadata/version/VersionPredicate.PredicateTerm.html#getReferenceVersion())
    pub fn getReferenceVersion<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::Version>>,
        ::java_spaghetti::Local<
            'env,
            super::super::super::super::super::super::java::lang::Throwable,
        >,
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
                        c"getReferenceVersion",
                        c"()Lnet/fabricmc/loader/api/Version;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
