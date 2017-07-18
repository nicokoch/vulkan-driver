gcc -DVK_USE_PLATFORM_XCB_KHR -lvulkan -lxcb triangle.c -g -o triangle
RUST_LOG=debug VK_ICD_FILENAMES="$HOME/dev/rust/vulkan-driver/icd/rusterizer_icd.json" VK_LOADER_DEBUG=all ./triangle
