import Html exposing (text)
-- import Json.Decode exposing (..)
import Json.Encode exposing (..)

type Format = PDF | DOC | DOCX | JPEG | TXT | ODG | ODT

encodeFormat : Format -> Value
encodeFormat format =
    object
    [("format",
         case format of
         PDF -> string "PDF"
         DOC -> string "DOC"
         DOCX -> string "DOCX"
         JPEG -> string "JPEG"
         TXT -> string "TXT"
         ODG -> string "ODG"
         ODT -> string "ODT")
    ]

type alias File =
  { path : String
  , format : Format
  }

type alias Document =
    { title: String
    , binder: String
    , folder: String
    , year: Int
    , file: File
    , tags: List String
    }

-- a = Document "ciao" "pollo" "asd" 10 (File "ciao" PDF) ["ciao", "asd"]

main =
  text (encode 2 (encodeFormat PDF))
