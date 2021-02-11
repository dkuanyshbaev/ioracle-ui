create table if not exists hexagrams (
    id integer not null primary key,
    binary text not null,
    king_wen_order integer not null,
    shao_yong_order integer not null,
    gua text not null,
    pin_yin text not null,
    character text not null,
    wilheim text not null,
    huang text not null,
    hatcher text not null,
    no2do text not null,
    inner_ba_gua text not null,
    outer_ba_gua text not null,
    host_yao text not null,
    judgment text not null,
    image text not null,
    lines text not null
)
