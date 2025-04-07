pub mod board_mapper {
    use sea_orm::ActiveValue::{NotSet, Set};
    use crate::domain::board::entity::command::board_entity::BoardEntity;
    use crate::domain::board::schema::board::{
        ActiveModel as ActiveBoardModel, Model as BoardModel,
    };

    pub fn to_domain(orm_board: &BoardModel) -> BoardEntity {
        BoardEntity::new(
            Some(orm_board.id),
            orm_board.name.to_owned(),
            Some(orm_board.created_at.to_owned()),
            orm_board.updated_at.to_owned(),
        )
    }

    pub fn to_orm(board_entity: &BoardEntity) -> ActiveBoardModel {
        ActiveBoardModel {
            id: if board_entity.get_id().is_some() {
                Set(board_entity.get_id().unwrap())
            } else {
                NotSet
            },
            name: Set(board_entity.get_name()),
            created_at: Set(board_entity.get_created_at()),
            updated_at: Set(board_entity.get_updated_at()),
        }
    }
}

pub mod post_mapper {

    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    use crate::domain::board::entity::command::post_entity::{PostEntity, PostEntityBuilder};
    use crate::domain::board::schema::post::{
        ActiveModel as ActivePostModel, Model as PostModel,
    };

    pub fn to_domain(orm_post: &PostModel) -> PostEntity {
        PostEntityBuilder::default()
            .id(Some(orm_post.id))
            .category_id(orm_post.category_id)
            .member_id(orm_post.member_id)
            .title(orm_post.title.to_owned())
            .contents(orm_post.contents.to_owned())
            .view_count(orm_post.view_count)
            .created_at(orm_post.created_at)
            .updated_at(orm_post.updated_at)
            .build()
            .unwrap()
    }

    pub fn to_orm(post_entity: &PostEntity) -> ActivePostModel {
        ActivePostModel {
            id: if post_entity.get_id().is_some() {
                Set(post_entity.get_id().unwrap())
            } else {
                Set(Uuid::new_v4())
            },
            member_id: Set(post_entity.get_member_id()),
            category_id: Set(post_entity.get_category_id()),
            title: Set(post_entity.get_title()),
            contents: Set(post_entity.get_contents()),
            view_count: Set(post_entity.get_view_count()),
            created_at: Set(post_entity.get_created_at()),
            updated_at: Set(post_entity.get_updated_at()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::board_mapper;
    use crate::domain::board::entity::command::board_entity::BoardEntity;
    use crate::domain::board::schema::board::Model as BoardModel;

    #[test]
    fn test_to_domain() {
        let orm_board = BoardModel {
            id: 1,
            name: "Test Board".to_string(),
            created_at: chrono::NaiveDateTime::from_timestamp(0, 0),
            updated_at: Some(chrono::NaiveDateTime::from_timestamp(0, 0)),
        };

        let domain_board = board_mapper::to_domain(&orm_board);

        assert_eq!(domain_board.get_id(), Some(1));
        assert_eq!(domain_board.get_name(), "Test Board");
    }

    #[test]
    fn test_to_orm() {
        let domain_board = BoardEntity::new(
            Some(1),
            "Test Board".to_string(),
            Some(chrono::NaiveDateTime::from_timestamp(0, 0)),
            None,
        );

        let orm_board = board_mapper::to_orm(&domain_board);

        assert_eq!(orm_board.id.unwrap(), 1);
        assert_eq!(orm_board.name.unwrap(), "Test Board");
    }
}
