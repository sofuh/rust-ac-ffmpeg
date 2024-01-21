#include "libavutil/dict.h"
#include <libavformat/avformat.h>

const AVDictionaryEntry* ffw_dict_advance_iterator(const AVDictionary *dict, const AVDictionaryEntry *prev) {
  if (dict == NULL) {
    return NULL;
  }

  return av_dict_iterate(dict, prev);
}

char *ffw_dict_entry_get_key(const AVDictionaryEntry *entry) {
  if (entry == NULL) {
    return NULL;
  }

  return entry->key;
}

char *ffw_dict_entry_get_value(const AVDictionaryEntry *entry) {
  if (entry == NULL) {
    return NULL;
  }

  return entry->value;
}
