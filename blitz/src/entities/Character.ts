import { BaseEntity, Column, Entity, PrimaryColumn } from 'typeorm';

@Entity('characters')
export class Character extends BaseEntity {
  @PrimaryColumn()
  character_id: string;

  @Column({
    nullable: true,
  })
  img: string;

  @Column({
    nullable: true,
  })
  tier: number;

  @Column({
    nullable: true,
  })
  rarity: number;

  @Column()
  display_name: string;

  @Column()
  square_icon_path: string;
}
