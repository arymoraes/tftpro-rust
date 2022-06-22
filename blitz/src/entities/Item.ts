import { BaseEntity, Column, Entity, PrimaryColumn } from 'typeorm';

@Entity('items')
export class Item extends BaseEntity {
  @PrimaryColumn()
  id: number;

  @Column({
    nullable: true,
  })
  img: string;

  @Column()
  name_id: string;

  @Column({
    nullable: true,
  })
  loadouts_icon: string;

  @Column({
    nullable: true,
  })
  guid: string;
}
