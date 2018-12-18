use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::*;
use std::fmt;
use std::io::Write;
use std::str;
use std::str::FromStr;
use utils::errors::EnumParseError;

macro_rules! string_enum {
    ($name:ident [$($value:ident),+]) => {

        #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Eq, Hash, FromSqlRow, AsExpression)]
        #[sql_type = "Text"]
        pub enum $name {
            $(
                $value,
            )*
        }

        impl ToSql<Text, Pg> for $name {
            fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
                match *self {
                    $(
                      $name::$value => out.write_all(stringify!($value).as_bytes())?,
                    )*
                }
                Ok(IsNull::No)
            }
        }

        impl FromSql<Text, Pg> for $name {
            fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
                match str::from_utf8(not_none!(bytes))? {
                    $(
                        stringify!($value) => Ok($name::$value),
                    )*
                    _ => Err("Unrecognized enum variant".into()),
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
             let s = match self {
                  $(
                    $name::$value => stringify!($value),
                   )*
                };
                write!(f, "{}", s)
            }
        }

        impl FromStr for $name {
            type Err = EnumParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
               $(
                  if s.eq_ignore_ascii_case(stringify!($value)) {
                     return Ok($name::$value);
                  }
               )*

               Err(EnumParseError {
                          message: "Could not parse value".to_string(),
                          enum_type: stringify!($name).to_string(),
                          value: s.to_string(),
                      })
            }
        }
    }
}

string_enum! { AssetStatus [Unsynced] }
string_enum! { CodeTypes [Access, Discount] }
string_enum! { CommunicationChannelType [Email, Sms, Push]}
string_enum! { DomainEventTypes [OrderBehalfOfUserChanged, PaymentCreated, PaymentCompleted, PaymentMethodCreated, PaymentMethodUpdated, UserRegistration, LostPassword, PurchaseCompleted]}
string_enum! { DomainActionTypes [Communication]}
string_enum! { DomainActionStatus [Pending, RetriesExceeded, Errored, Success, Cancelled]}
string_enum! { EventStatus [Draft,Closed,Published,Offline]}
string_enum! { EventSearchSortField [ Name, EventStart]}
string_enum! { EventOverrideStatus [PurchaseTickets,SoldOut,OnSaleSoon,TicketsAtTheDoor,Free,Rescheduled,Cancelled,OffSale,Ended]}
string_enum! { FanSortField [FirstName, LastName, Email, Phone, Orders, FirstOrder, LastOrder, Revenue] }
string_enum! { HistoryType [Purchase]}
string_enum! { HoldTypes [Discount, Comp] }
string_enum! { OrderStatus [Draft, PartiallyPaid, Paid, Cancelled] }
string_enum! { OrderItemTypes [Tickets, PerUnitFees, EventFees]}
string_enum! { OrderTypes [Cart, BackOffice] }
string_enum! { PaymentMethods [External, CreditCard] }
string_enum! { PaymentStatus [Authorized, Completed] }
string_enum! { PastOrUpcoming [Past,Upcoming]}
string_enum! { Roles [Admin, OrgMember, OrgOwner, OrgAdmin, OrgBoxOffice, DoorPerson, User] }
string_enum! { SortingDir[ Asc, Desc ] }
string_enum! { Tables [Orders, Payments, PaymentMethods] }
string_enum! { TicketInstanceStatus [Available, Reserved, Purchased, Redeemed, Nullified]}
string_enum! { TicketPricingStatus [Published, Deleted] }
string_enum! { TicketTypeStatus [NoActivePricing, Published, SoldOut] }

impl Default for EventStatus {
    fn default() -> EventStatus {
        EventStatus::Draft
    }
}

#[test]
fn display() {
    assert_eq!(Roles::Admin.to_string(), "Admin");
    assert_eq!(Roles::OrgAdmin.to_string(), "OrgAdmin");
    assert_eq!(Roles::OrgMember.to_string(), "OrgMember");
    assert_eq!(Roles::OrgOwner.to_string(), "OrgOwner");
    assert_eq!(Roles::OrgBoxOffice.to_string(), "OrgBoxOffice");
    assert_eq!(Roles::DoorPerson.to_string(), "DoorPerson");
    assert_eq!(Roles::User.to_string(), "User");
}

#[test]
fn parse() {
    assert_eq!(Roles::Admin, "Admin".parse().unwrap());
    assert_eq!(Roles::OrgMember, "OrgMember".parse().unwrap());
    assert_eq!(Roles::OrgOwner, "OrgOwner".parse().unwrap());
    assert_eq!(Roles::OrgBoxOffice, "OrgBoxOffice".parse().unwrap());
    assert!("Invalid Role".parse::<Roles>().is_err());
}
