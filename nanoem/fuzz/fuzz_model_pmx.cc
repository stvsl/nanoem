#include "./common.h"

extern "C" int
LLVMFuzzerTestOneInput(const uint8_t *data, size_t size)
{
    nanoem_status_t status = NANOEM_STATUS_SUCCESS;
    nanoem_unicode_string_factory_t *factory = nanoemUnicodeStringFactoryCreateEXT(&status);
    nanoem_model_t *model = nanoemModelCreate(factory, &status);
    nanoem_buffer_t *buffer = nanoemBufferCreate(data, size, &status);
    nanoemModelLoadFromBufferPMX(model, buffer, &status);
    nanoemModelDestroy(model);
    nanoemBufferDestroy(buffer);
    nanoemUnicodeStringFactoryDestroyEXT(factory);
    return 0;
}
