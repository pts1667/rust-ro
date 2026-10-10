pub mod advanced_refiner;
pub mod alchemist;
pub mod buying_shops;
pub mod cashheadgear_dye;
pub mod clothes_dyer;
pub mod coin_exchange;
pub mod dye_maker;
pub mod elemental_trader;
pub mod enchan_arm;
pub mod gemstone;
pub mod hair_dyer;
pub mod hair_style;
pub mod icecream;
pub mod inn;
pub mod kunai_maker;
pub mod milk_trader;
pub mod novice_exchange;
pub mod old_pharmacist;
pub mod quivers;
pub mod refine;
pub mod renters;
pub mod socket_enchant;
pub mod socket_enchant2;
pub mod wander_pet_food;

script_sdk_2::script_module! {
    npcs {
        "Abdula" => refine::abdula,
        "Alchemist#ama" => elemental_trader::alchemist_ama,
        "Alora" => cashheadgear_dye::alora,
        "Antonio" => refine::antonio,
        "Apprentice Craftsman" => enchan_arm::apprentice_craftsman,
        "Aragham" => refine::aragham,
        "Assistant Beautician#li" => hair_style::assistant_beautician_li,
        "Begnahd" => refine::begnahd,
        "Berry Toe" => wander_pet_food::berry_toe,
        "Black Marketeer#Buying" => buying_shops::black_marketeer_buying,
        "Christopher#1" => refine::christopher_1,
        "Delight" => refine::delight,
        "Dietrich" => refine::dietrich,
        "Dilemma" => refine::dilemma,
        "Dyer Ginedin Rephere" => clothes_dyer::dyer_ginedin_rephere,
        "Falcon Breeder#hnt" => renters::falcon_breeder_hnt,
        "Fredrik" => refine::fredrik,
        "Fruel" => refine::fruel,
        "Fulerr" => refine::fulerr,
        "Guild Dealer" => alchemist::guild_dealer,
        "Hair Dyer#lich" => hair_dyer::hair_dyer_lich,
        "Hakhim" => refine::hakhim,
        "Hollgrehenn" => refine::hollgrehenn,
        "Hotel Employee#01" => inn::hotel_employee_01,
        "Hotel Employee#ein" => inn::hotel_employee_ein,
        "Hotel Keeper#bra1" => inn::hotel_keeper_bra1,
        "Ice Cream Maker" => icecream::ice_cream_maker,
        "Inn Employee#Ahee" => inn::inn_employee_ahee,
        "Inn Employee#Ahlma" => inn::inn_employee_ahlma,
        "Inn Employee#Cena" => inn::inn_employee_cena,
        "Inn Employee#Jennie" => inn::inn_employee_jennie,
        "Inn Employee#Sammy" => inn::inn_employee_sammy,
        "Inn Keeper#Annie" => inn::inn_keeper_annie,
        "Inn Maid#Receptionist" => inn::inn_maid_receptionist,
        "Inn Maid#Rilim" => inn::inn_maid_rilim,
        "Inn Master#Receptionist" => inn::inn_master_receptionist,
        "Inventor Jaax" => quivers::inventor_jaax,
        "Jade#pay" => gemstone::jade_pay,
        "Java Dullihan" => dye_maker::java_dullihan,
        "Kahlamanlith" => refine::kahlamanlith,
        "Krugg" => refine::krugg,
        "Kunai Merchant Kashin" => kunai_maker::kunai_merchant_kashin,
        "Lambert" => refine::lambert,
        "Leablem#dummy" => socket_enchant2::leablem_dummy,
        "Manthasman" => refine::manthasman,
        "Matestein" => refine::matestein,
        "Merchant#Morocc" => novice_exchange::merchant_morocc,
        "Merchant#alde" => novice_exchange::merchant_alde,
        "Merchant#geff" => novice_exchange::merchant_geff,
        "Merchant#pay" => novice_exchange::merchant_pay,
        "Merchant#pron" => novice_exchange::merchant_pron,
        "Milk Vendor" => milk_trader::milk_vendor,
        "Paul Spanner" => refine::paul_spanner,
        "Peco Peco Breeder#cru" => renters::peco_peco_breeder_cru,
        "Peco Peco Breeder#knt" => renters::peco_peco_breeder_knt,
        "Pet Enthusiast" => wander_pet_food::pet_enthusiast,
        "Pharmacist" => old_pharmacist::pharmacist,
        "Purchasing Team#Buying" => buying_shops::purchasing_team_buying,
        "Repairman#alb" => refine::repairman_alb,
        "Repairman#alde" => refine::repairman_alde,
        "Repairman#alde_gld" => refine::repairman_alde_gld,
        "Repairman#aru_gld" => refine::repairman_aru_gld,
        "Repairman#gef" => refine::repairman_gef,
        "Repairman#gef_fild" => refine::repairman_gef_fild,
        "Repairman#juno" => refine::repairman_juno,
        "Repairman#lhz" => refine::repairman_lhz,
        "Repairman#moc" => refine::repairman_moc,
        "Repairman#pay" => refine::repairman_pay,
        "Repairman#pay_gld" => refine::repairman_pay_gld,
        "Repairman#prt" => refine::repairman_prt,
        "Repairman#prt_gld" => refine::repairman_prt_gld,
        "Repairman#sch_gld" => refine::repairman_sch_gld,
        "Roving Hair Dresser" => hair_style::roving_hair_dresser,
        "Sade" => refine::sade,
        "Seiyablem#dummy" => socket_enchant::seiyablem_dummy,
        "Suhnbi#cash" => advanced_refiner::suhnbi_cash,
        "Tirehaus" => refine::tirehaus,
        "Vurewell" => refine::vurewell,
        "Xenophon" => refine::xenophon,
        "Young Man#dummy" => socket_enchant::young_man_dummy,
        "merchant_13_2" => coin_exchange::merchant_13_2,
    }
    events {
    }
}
