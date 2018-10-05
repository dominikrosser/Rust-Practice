module Main exposing (Book, Model, Msg(..), bookDecoder, bookView, boolToString, decodeBooks, getBooks, init, main, subscriptions, update, view)

import Browser exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Json
import Json.Encode as Encode


type alias Book =
    { id : Int
    , title : String
    , author : String
    , published : Bool
    }


type alias Model =
    { books : List Book
    , title : String
    , author : String
    , published : Bool
    , errorMsg : String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Model [] "" "" False "", Cmd.none )


type Msg
    = GetBooks (Result Http.Error (List Book))
    | SetBook (Result Http.Error String)
    | DeleteBook (Result Http.Error String)
    | RequestBooks
    | PostBook
    | DelBook Int
    | GetTitle String
    | GetAuthor String
    | GetPublished


httpBookCompleted : Model -> Result Http.Error String -> ( Model, Cmd Msg )
httpBookCompleted model result =
    case result of
        Ok string ->
            ( { model | errorMsg = "" } |> Debug.log "Status Complete", Cmd.none )

        Err errorHttp ->
            ( { model | errorMsg = "Error...TODO convert error to string" }, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GetBooks (Ok json) ->
            ( { model | books = json }, Cmd.none )

        GetBooks (Err err) ->
            ( Debug.log "An error occured while getting the books" model, Cmd.none )

        RequestBooks ->
            ( model, getBooks )

        SetBook result ->
            httpBookCompleted model result

        DeleteBook result ->
            httpBookCompleted model result

        PostBook ->
            ( { model | title = "", author = "" }, bookPostCmd model )

        DelBook i ->
            ( { model | books = List.filter (\b -> b.id /= i) model.books }, bookDelCmd i )

        GetTitle s ->
            ( { model | title = s }, Cmd.none )

        GetAuthor s ->
            ( { model | author = s }, Cmd.none )

        GetPublished ->
            ( { model | published = not model.published }, Cmd.none )


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
    Json.map4
        Book
        (Json.at [ "id" ] Json.int)
        (Json.at [ "title" ] Json.string)
        (Json.at [ "author" ] Json.string)
        (Json.at [ "published" ] Json.bool)


bookPostCmd : Model -> Cmd Msg
bookPostCmd model =
    Http.send SetBook (addBook model)


addBook : Model -> Http.Request String
addBook model =
    let
        url =
            "http://localhost:8000/api/v1/books"

        body =
            model
                |> bookEncoder
                |> Http.jsonBody
    in
    Http.post url body statusDecoder


bookEncoder : Model -> Encode.Value
bookEncoder model =
    Encode.object
        [ ( "title", Encode.string model.title )
        , ( "author", Encode.string model.author )
        , ( "published", Encode.bool model.published )
        ]


statusDecoder : Json.Decoder String
statusDecoder =
    Json.field "status" Json.string


bookDelCmd : Int -> Cmd Msg
bookDelCmd i =
    let
        decoder =
            Json.succeed ""

        a =
            i |> String.fromInt

        request =
            Http.request
                { method = "DELETE"
                , headers = []
                , url = "http://localhost:8000/api/v1/books/" ++ a
                , body = Http.emptyBody
                , expect = Http.expectJson decoder
                , timeout = Maybe.Nothing
                , withCredentials = False
                }
    in
    Http.send DeleteBook request


view : Model -> Html Msg
view model =
    div []
        [ div [] <| List.map bookView model.books
        , button [ onClick RequestBooks ] [ text "Get Books!" ]
        , bookForm model
        ]


bookView : Book -> Html Msg
bookView book =
    ul []
        [ li [] [ text book.title ]
        , li [] [ text book.author ]
        , li [] [ book.published |> boolToString |> text ]
        , button [ onClick (DelBook book.id) ] [ text "X" ]
        ]


bookForm : Model -> Html Msg
bookForm model =
    div [ id "form" ]
        [ label [ for "title" ] [ text " Title " ]
        , input [ id "title", type_ "text", Html.Attributes.value model.title, onInput GetTitle ] []
        , label [ for "author" ] [ text " Author " ]
        , input [ id "author", type_ "text", Html.Attributes.value model.author, onInput GetAuthor ] []
        , label [ for "published" ] [ text " Published " ]
        , input [ id "published ", type_ "checkbox", onClick GetPublished ] []
        , button [ onClick PostBook ] [ text "Post Book!" ]
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
