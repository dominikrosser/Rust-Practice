module Main exposing (Book, Model, Msg(..), bookDecoder, bookView, boolToString, decodeBooks, getBooks, init, main, subscriptions, update, view)

import Browser exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Json
import Json.Encode as Encode


type alias Book =
    { title : String
    , author : String
    , published : Bool
    }


type alias Model =
    { books : List Book }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Model [], Cmd.none )


type Msg
    = GetBooks (Result Http.Error (List Book))
    | RequestBooks


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GetBooks (Ok json) ->
            ( { model | books = json }, Cmd.none )

        GetBooks (Err err) ->
            ( Debug.log "An error occured while getting the books" model, Cmd.none )

        RequestBooks ->
            ( model, getBooks )


getBooks : Cmd Msg
getBooks =
    let
        url =
            "http://localhost:8000/api/v1/books"

        req =
            Http.get url decodeBooks
    in
    Http.send GetBooks req


decodeBooks : Json.Decoder (List Book)
decodeBooks =
    Json.at [ "result" ] (Json.list bookDecoder)


bookDecoder : Json.Decoder Book
bookDecoder =
    Json.map3
        Book
        (Json.at [ "title" ] Json.string)
        (Json.at [ "author" ] Json.string)
        (Json.at [ "published" ] Json.bool)


view : Model -> Html Msg
view model =
    div []
        [ div [] <| List.map bookView model.books
        , button [ onClick RequestBooks ] [ text "Get Books!" ]
        , text "Hello World"
        ]


bookView : Book -> Html Msg
bookView book =
    ul []
        [ li [] [ text book.title ]
        , li [] [ text book.author ]
        , li [] [ book.published |> boolToString |> text ]
        ]


boolToString : Bool -> String
boolToString b =
    if b == True then
        "True"

    else
        "False"


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }
