package io.github.pitchblacknights.oxidizemc;

import com.google.common.io.ByteStreams;
import net.fabricmc.api.ModInitializer;
import net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents;
import net.fabricmc.loader.api.FabricLoader;
import net.minecraft.server.MinecraftServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.slf4j.event.Level;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Comparator;
import java.util.Locale;
import java.util.stream.Stream;

public class OxidizeMC implements ModInitializer {
    public static final String MOD_ID = "oxidizemc";
    // FIELD: Lorg/slf4j/Logger
    // METHOD: (Ljava/lang/String;)V;
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

    private Path configDirectory;

    public static void deleteDirectory(String dir) throws IOException {
        Path path = Paths.get(dir);
        try (Stream<Path> walk = Files.walk(path)) {
            walk.sorted(Comparator.reverseOrder()).forEach(OxidizeMC::deleteDirectoryExtract);
        }
    }

    public static void deleteDirectoryExtract(Path path) {
        try {
            Files.delete(path);
        } catch (IOException e) {
            System.err.printf("Unable to delete this path : %s%n%s", path, e);
        }
    }

    public static native String returnTest(int num);

    public static native void testLogString(String text);

    public static native void testLogLevel(Level level, String text);

    private static String getLibrary(String os, String arch) {
        String lib = null;
        if (os.contains("win")) {
            if (arch.equals("x86_64") || arch.equals("amd64")) {
                lib = "oxidizemc-win-x86_64.dll";
            } else if (arch.equals("aarch64") || arch.equals("arm64")) {
                lib = "oxidizemc-win-aarch64.dll";
            }
        } else if (os.contains("linux")) {
            if (arch.equals("x86_64") || arch.equals("amd64")) {
                lib = "oxidizemc-linux-x86_64.so";
            } else if (arch.equals("aarch64") || arch.equals("arm64")) {
                lib = "oxidizemc-linux-aarch64.so";
            }
        } else if (os.contains("mac")) {
            if (arch.equals("x86_64") || arch.equals("amd64")) {
                lib = "oxidizemc-osx-x86_64.dylib";
            } else if (arch.equals("aarch64") || arch.equals("arm64")) {
                lib = "oxidizemc-osx-x86_64.dylib";
            }
        }
        return lib;
    }

    @Override
    public void onInitialize() {
        LOGGER.info("Initializing mod...");

        FabricLoader loader = FabricLoader.getInstance();
        this.configDirectory = loader.getConfigDir().resolve("oxidizemc");
        ServerLifecycleEvents.SERVER_STOPPING.register(this::onServerStopping);

        String os = System.getProperty("os.name").toLowerCase(Locale.ENGLISH);
        String arch = System.getProperty("os.arch").toLowerCase(Locale.ENGLISH);
        String lib = getLibrary(os, arch);
        if (lib == null) {
            throw new UnsupportedSystemException(os, arch);
        }

        LOGGER.info("System: {}/{}", os, arch);
        LOGGER.info("Library: {}", lib);

        String libPath = "rust_lib/" + lib;
        URL libResource = this.getClass().getClassLoader().getResource(libPath);
        if (libResource == null) {
            throw new RuntimeException("Could not find resource '" + libPath + "'");
        }

        Path extractPath = configDirectory.resolve("temp").toAbsolutePath();
        //noinspection ResultOfMethodCallIgnored
        extractPath.toFile().mkdirs();
        extractPath = Paths.get(extractPath + "/" + lib);

        try (InputStream in = libResource.openStream(); OutputStream out = Files.newOutputStream(extractPath)) {
            ByteStreams.copy(in, out);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }

        System.load(extractPath.toAbsolutePath().toString());

        LOGGER.info("Returned value: {}", returnTest(15));
        testLogString("Hello World!");
        //        // TRACE_INT = 00;
//        // DEBUG_INT = 10;
//        // INFO_INT = 20;
//        // WARN_INT = 30;
//        // ERROR_INT = 40;
//        testLogLevel(Level.TRACE, "Trace Log");
//        testLogLevel(Level.DEBUG, "Debug Log");
//        testLogLevel(Level.INFO, "Info Log");
//        testLogLevel(Level.WARN, "Warn Log");
//        testLogLevel(Level.ERROR, "Error Log");
    }

    public void onServerStopping(MinecraftServer server) {
        LOGGER.info("Disabling mod...");
        try {
            deleteDirectory(String.valueOf(configDirectory.resolve("temp")));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private static final class UnsupportedSystemException extends RuntimeException {
        public UnsupportedSystemException(String os, String arch) {
            super("Your system configuration is not supported by OxidizeMC: " + os + "/" + arch);
        }
    }

    private static final class LibraryLoadingException extends RuntimeException {
        public LibraryLoadingException(Throwable cause) {
            super("OxidizeMC encountered an error while loading a mod", cause);
        }
    }
}
