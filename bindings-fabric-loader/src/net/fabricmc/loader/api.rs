pub mod entrypoint;
pub mod metadata;
///class [EntrypointException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/EntrypointException.html)
pub enum EntrypointException {}
unsafe impl ::java_spaghetti::ReferenceType for EntrypointException {}
unsafe impl ::java_spaghetti::JniType for EntrypointException {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/EntrypointException")
    }
}
impl EntrypointException {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/EntrypointException"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///interface [FabricLoader](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/FabricLoader.html)
pub enum FabricLoader {}
unsafe impl ::java_spaghetti::ReferenceType for FabricLoader {}
unsafe impl ::java_spaghetti::JniType for FabricLoader {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/FabricLoader")
    }
}
impl FabricLoader {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/FabricLoader"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getInstance](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/FabricLoader.html#getInstance())
    pub fn getInstance<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, FabricLoader>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getInstance",
                        c"()Lnet/fabricmc/loader/api/FabricLoader;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getObjectShare](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/FabricLoader.html#getObjectShare())
    pub fn getObjectShare<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, ObjectShare>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getObjectShare",
                        c"()Lnet/fabricmc/loader/api/ObjectShare;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getMappingResolver](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/FabricLoader.html#getMappingResolver())
    pub fn getMappingResolver<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, MappingResolver>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getMappingResolver",
                        c"()Lnet/fabricmc/loader/api/MappingResolver;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[isDevelopmentEnvironment](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/FabricLoader.html#isDevelopmentEnvironment())
    pub fn isDevelopmentEnvironment<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"isDevelopmentEnvironment",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getEnvironmentType](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/FabricLoader.html#getEnvironmentType())
    pub fn getEnvironmentType<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::super::api::EnvType>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getEnvironmentType",
                        c"()Lnet/fabricmc/api/EnvType;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [LanguageAdapter](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/LanguageAdapter.html)
pub enum LanguageAdapter {}
unsafe impl ::java_spaghetti::ReferenceType for LanguageAdapter {}
unsafe impl ::java_spaghetti::JniType for LanguageAdapter {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/LanguageAdapter")
    }
}
impl LanguageAdapter {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/LanguageAdapter"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getDefault](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/LanguageAdapter.html#getDefault())
    pub fn getDefault<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, LanguageAdapter>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getDefault",
                        c"()Lnet/fabricmc/loader/api/LanguageAdapter;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
}
///class [LanguageAdapterException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/LanguageAdapterException.html)
pub enum LanguageAdapterException {}
unsafe impl ::java_spaghetti::ReferenceType for LanguageAdapterException {}
unsafe impl ::java_spaghetti::JniType for LanguageAdapterException {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/LanguageAdapterException")
    }
}
impl LanguageAdapterException {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/LanguageAdapterException"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///interface [MappingResolver](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/MappingResolver.html)
pub enum MappingResolver {}
unsafe impl ::java_spaghetti::ReferenceType for MappingResolver {}
unsafe impl ::java_spaghetti::JniType for MappingResolver {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/MappingResolver")
    }
}
impl MappingResolver {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/MappingResolver"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///interface [ModContainer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/ModContainer.html)
pub enum ModContainer {}
unsafe impl ::java_spaghetti::ReferenceType for ModContainer {}
unsafe impl ::java_spaghetti::JniType for ModContainer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/ModContainer")
    }
}
impl ModContainer {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/ModContainer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getMetadata](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/ModContainer.html#getMetadata())
    pub fn getMetadata<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, metadata::ModMetadata>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getMetadata",
                        c"()Lnet/fabricmc/loader/api/metadata/ModMetadata;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getOrigin](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/ModContainer.html#getOrigin())
    pub fn getOrigin<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, metadata::ModOrigin>>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getOrigin",
                        c"()Lnet/fabricmc/loader/api/metadata/ModOrigin;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [ObjectShare](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/ObjectShare.html)
pub enum ObjectShare {}
unsafe impl ::java_spaghetti::ReferenceType for ObjectShare {}
unsafe impl ::java_spaghetti::JniType for ObjectShare {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/ObjectShare")
    }
}
impl ObjectShare {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/ObjectShare"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///interface [SemanticVersion](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/SemanticVersion.html)
pub enum SemanticVersion {}
unsafe impl ::java_spaghetti::ReferenceType for SemanticVersion {}
unsafe impl ::java_spaghetti::JniType for SemanticVersion {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/SemanticVersion")
    }
}
unsafe impl ::java_spaghetti::AssignableTo<Version> for SemanticVersion {}
impl SemanticVersion {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/SemanticVersion"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getVersionComponentCount](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/SemanticVersion.html#getVersionComponentCount())
    pub fn getVersionComponentCount<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        i32,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getVersionComponentCount",
                        c"()I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getVersionComponent](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/SemanticVersion.html#getVersionComponent(int))
    pub fn getVersionComponent<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: i32,
    ) -> ::std::result::Result<
        i32,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"getVersionComponent",
                        c"(I)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[hasWildcard](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/SemanticVersion.html#hasWildcard())
    pub fn hasWildcard<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        bool,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"hasWildcard",
                        c"()Z",
                    ))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[compareTo](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/SemanticVersion.html#compareTo(net.fabricmc.loader.api.SemanticVersion))
    #[deprecated]
    pub fn compareTo<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
        arg0: impl ::java_spaghetti::AsArg<SemanticVersion>,
    ) -> ::std::result::Result<
        i32,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
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
                        c"compareTo",
                        c"(Lnet/fabricmc/loader/api/SemanticVersion;)I",
                    ))
                })
                .as_raw();
            __jni_env.call_int_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///public static final [COMPONENT_WILDCARD](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/SemanticVersion.html#COMPONENT_WILDCARD)
    pub const COMPONENT_WILDCARD: i32 = -2147483648;
}
///interface [Version](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/Version.html)
pub enum Version {}
unsafe impl ::java_spaghetti::ReferenceType for Version {}
unsafe impl ::java_spaghetti::JniType for Version {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/Version")
    }
}
impl Version {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/Version"),
                )
                .as_global()
            })
            .as_raw()
    }
}
///class [VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/VersionParsingException.html)
pub enum VersionParsingException {}
unsafe impl ::java_spaghetti::ReferenceType for VersionParsingException {}
unsafe impl ::java_spaghetti::JniType for VersionParsingException {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/VersionParsingException")
    }
}
impl VersionParsingException {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env.require_class(c"net/fabricmc/loader/api/VersionParsingException"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[VersionParsingException](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/VersionParsingException.html#VersionParsingException())
    pub fn new<'env>(
        __jni_env: ::java_spaghetti::Env<'env>,
    ) -> ::std::result::Result<
        ::java_spaghetti::Local<'env, Self>,
        ::java_spaghetti::Local<'env, super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"<init>",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.new_object_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
}
