import { BaseEntity, Column, Entity, PrimaryColumn } from 'typeorm';

@Entity('traits')
export class Trait extends BaseEntity {
  @PrimaryColumn()
  trait_id: string;

  @Column()
  display_name: string;

  @Column()
  set: string;

  @Column()
  icon_path: string;

  @Column({
    nullable: true,
  })
  img: string;
}
