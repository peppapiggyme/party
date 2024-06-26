#pragma once

#include <cinttypes>
#include <fstream>

/**
 * @brief File header of TFile
 *
 * @see https://root.cern.ch/doc/master/classTFile.html
 */
struct FileHeader_t {
    int32_t identifier;   // Root file identifier
    int32_t f_version;    // File format version
    int32_t f_begin;      // Pointer to first data record
    int32_t f_end;        // Pointer to first free word at the EOF
    int32_t f_seekfree;   // Pointer to FREE data record
    int32_t f_nbytesfree; // Number of bytes in FREE data record
    int32_t nfree;        // Number of free data records
    int32_t f_nbytesname; // Number of bytes in TNamed at creation time
    int8_t  f_units;      // Number of bytes for file pointers
    int32_t f_compress;   // Compression level and algorithm
    int32_t f_seekinfo;   // Pointer to TStreamerInfo record
    int32_t f_nbytesinfo; // Number of bytes in TStreamerInfo record
    int8_t  f_uuid[18];   // Universal Unique ID

    /**
     * @brief Read field values from file stream
     *
     * @param fs file stream
     */
    void read(std::ifstream& fs);
};

std::ostream& operator<<(std::ostream& os, const FileHeader_t& fh);